//! RPoRea の C ABI。**どの言語からも同じ共有ライブラリ(`rporea_ffi.dll` / `librporea_ffi.so`)を呼ぶ**:
//! Python(`ctypes`)・Mojo(FFI)・RMojo(Python構文)・C/C++ など。Rust からは `rporea` を直接使う。
//!
//! 公開するもの(まず最小): 静的な応答(HTML・JSON・画像など)の登録、Connect メソッド
//! (`POST /pkg.Service/Method`)のコールバック登録、起動。ページを部品(VNode)で組む Reaxt の機能は、Rust 側から使う。
//!
//! 約束(安全):
//! - すべての関数は、ヌルや不正なUTF-8を受けても落ちず、負の値(エラー)を返す。
//! - `Builder` は1つのスレッドから使う。`rporea_start`/`rporea_serve` は `Builder` を消費する(その後は使えない)。
//! - コールバックはサーバーのワーカースレッドから呼ばれる。スレッド安全性は呼び出し側(C/Python等)の責任。
use rporea::reaxt::app::App;
use rporea::reaxt::backend::{Denied, Module};
use std::ffi::{c_char, c_void, CStr};
use std::sync::Arc;

/// 不透明なハンドル(C側は中身を見ない)。
pub struct Builder {
    app: Option<App>,
}

pub const RPOREA_OK: i32 = 0;
pub const RPOREA_E_NULL: i32 = -1;
pub const RPOREA_E_UTF8: i32 = -2;
pub const RPOREA_E_CONSUMED: i32 = -3;
pub const RPOREA_E_START: i32 = -4;

/// コールバック: `body`(JSON、UTF-8、`body_len`バイト)を受け、`out`(容量`out_cap`)へ応答を書き、
/// 書いた長さを返す。負の値は失敗(`-403` ならHTTP 403、400〜599の範囲に丸める)。
pub type MethodCallback =
    extern "C" fn(user: *mut c_void, body: *const u8, body_len: usize, out: *mut u8, out_cap: usize) -> isize;

/// 応答の最大サイズ(本文の上限と同じ 1 MiB)。
const OUT_CAP: usize = 1 << 20;

struct UserPtr(*mut c_void);
// 呼び出し先(C側)がスレッド安全であることは、呼び出す側の責任(ヘッダに明記)。
unsafe impl Send for UserPtr {}
unsafe impl Sync for UserPtr {}

fn s<'a>(p: *const c_char) -> Result<&'a str, i32> {
    if p.is_null() {
        return Err(RPOREA_E_NULL);
    }
    unsafe { CStr::from_ptr(p) }.to_str().map_err(|_| RPOREA_E_UTF8)
}

#[no_mangle]
pub extern "C" fn rporea_version() -> *const c_char {
    static V: &str = concat!(env!("CARGO_PKG_VERSION"), "\0");
    V.as_ptr() as *const c_char
}

/// `title` はページの `<title>`。失敗時は NULL。
#[no_mangle]
pub extern "C" fn rporea_new(title: *const c_char) -> *mut Builder {
    match s(title) {
        Ok(t) => Box::into_raw(Box::new(Builder { app: Some(App::new(t)) })),
        Err(_) => std::ptr::null_mut(),
    }
}

/// 使わずに捨てる。
#[no_mangle]
pub extern "C" fn rporea_free(b: *mut Builder) {
    if !b.is_null() {
        drop(unsafe { Box::from_raw(b) });
    }
}

fn with_app(b: *mut Builder, f: impl FnOnce(App) -> App) -> i32 {
    if b.is_null() {
        return RPOREA_E_NULL;
    }
    let b = unsafe { &mut *b };
    match b.app.take() {
        Some(app) => {
            b.app = Some(f(app));
            RPOREA_OK
        }
        None => RPOREA_E_CONSUMED,
    }
}

/// 静的な応答を `GET path` で返す。`content_type` は例 `text/html; charset=utf-8`。
#[no_mangle]
pub extern "C" fn rporea_asset(
    b: *mut Builder,
    path: *const c_char,
    content_type: *const c_char,
    data: *const u8,
    len: usize,
) -> i32 {
    let (path, ct) = match (s(path), s(content_type)) {
        (Ok(p), Ok(c)) => (p.to_string(), c.to_string()),
        (Err(e), _) | (_, Err(e)) => return e,
    };
    if data.is_null() && len > 0 {
        return RPOREA_E_NULL;
    }
    let bytes = if len == 0 { Vec::new() } else { unsafe { std::slice::from_raw_parts(data, len) }.to_vec() };
    // `App::asset` は `&'static str` を取る。登録ごとに1回だけ確保して保持する(登録数は小さい)。
    let ct: &'static str = Box::leak(ct.into_boxed_str());
    with_app(b, |a| a.asset(&path, ct, bytes))
}

/// Connect メソッド(`POST /service/method`、`service` は `pkg.Service` 形式)をコールバックで処理する。
#[no_mangle]
pub extern "C" fn rporea_method(
    b: *mut Builder,
    service: *const c_char,
    method: *const c_char,
    cb: Option<MethodCallback>,
    user: *mut c_void,
) -> i32 {
    let (service, method) = match (s(service), s(method)) {
        (Ok(a), Ok(b)) => (a.to_string(), b.to_string()),
        (Err(e), _) | (_, Err(e)) => return e,
    };
    let Some(cb) = cb else { return RPOREA_E_NULL };
    let user = Arc::new(UserPtr(user));
    with_app(b, move |a| {
        a.module(Module::new().method(&service, &method, move |_ctx, body| {
            let mut out = vec![0u8; OUT_CAP];
            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                cb(user.0, body.as_ptr(), body.len(), out.as_mut_ptr(), out.len())
            }));
            match r {
                Ok(n) if n >= 0 && (n as usize) <= OUT_CAP => {
                    out.truncate(n as usize);
                    String::from_utf8(out).map_err(|_| Denied::new(500, "callback returned invalid UTF-8"))
                }
                Ok(n) if n < 0 => Err(Denied::new((-n).clamp(400, 599) as u16, "callback error")),
                _ => Err(Denied::new(500, "callback failed")),
            }
        }))
    })
}

fn take_app(b: *mut Builder) -> Result<App, i32> {
    if b.is_null() {
        return Err(RPOREA_E_NULL);
    }
    let mut b = unsafe { Box::from_raw(b) }; // 消費する
    b.app.take().map(|a| a.finish()).ok_or(RPOREA_E_CONSUMED)
}

/// バックグラウンドで起動し、実際に bind したポート(正の値)を返す。`port=0` なら空きポート。失敗は負の値。
/// `Builder` は消費される。サーバーはプロセスが終わるまで動く。
#[no_mangle]
pub extern "C" fn rporea_start(b: *mut Builder, port: u16) -> i32 {
    let app = match take_app(b) {
        Ok(a) => Arc::new(a),
        Err(e) => return e,
    };
    let (tx, rx) = std::sync::mpsc::channel::<i32>();
    std::thread::spawn(move || {
        let rt = match tokio::runtime::Builder::new_multi_thread().enable_all().build() {
            Ok(r) => r,
            Err(_) => {
                let _ = tx.send(RPOREA_E_START);
                return;
            }
        };
        rt.block_on(async move {
            match rporea::start(app, ([127, 0, 0, 1], port)).await {
                Ok((addr, handle)) => {
                    let _ = tx.send(addr.port() as i32);
                    let _ = handle.await;
                }
                Err(_) => {
                    let _ = tx.send(RPOREA_E_START);
                }
            }
        });
    });
    rx.recv().unwrap_or(RPOREA_E_START)
}

/// 起動して、サーバーが終わるまで戻らない。`Builder` は消費される。
#[no_mangle]
pub extern "C" fn rporea_serve(b: *mut Builder, port: u16) -> i32 {
    let app = match take_app(b) {
        Ok(a) => Arc::new(a),
        Err(e) => return e,
    };
    let rt = match tokio::runtime::Builder::new_multi_thread().enable_all().build() {
        Ok(r) => r,
        Err(_) => return RPOREA_E_START,
    };
    match rt.block_on(rporea::serve(app, ([127, 0, 0, 1], port))) {
        Ok(()) => RPOREA_OK,
        Err(_) => RPOREA_E_START,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::io::{Read, Write};

    extern "C" fn echo(_user: *mut c_void, body: *const u8, len: usize, out: *mut u8, cap: usize) -> isize {
        let b = unsafe { std::slice::from_raw_parts(body, len) };
        let reply = format!("{{\"echo\":{}}}", String::from_utf8_lossy(b));
        if reply.len() > cap {
            return -500;
        }
        unsafe { std::ptr::copy_nonoverlapping(reply.as_ptr(), out, reply.len()) };
        reply.len() as isize
    }
    extern "C" fn deny(_: *mut c_void, _: *const u8, _: usize, _: *mut u8, _: usize) -> isize {
        -403
    }

    fn http(port: i32, req: &str) -> String {
        let mut s = std::net::TcpStream::connect(("127.0.0.1", port as u16)).unwrap();
        s.write_all(req.as_bytes()).unwrap();
        let mut out = String::new();
        s.read_to_string(&mut out).unwrap();
        out
    }

    #[test]
    fn c_abi_serves_asset_and_callback_methods_and_rejects_bad_input() {
        let title = CString::new("ffi").unwrap();
        let b = rporea_new(title.as_ptr());
        assert!(!b.is_null());
        let (p, ct) = (CString::new("/hello.txt").unwrap(), CString::new("text/plain; charset=utf-8").unwrap());
        assert_eq!(rporea_asset(b, p.as_ptr(), ct.as_ptr(), b"hi".as_ptr(), 2), RPOREA_OK);
        let (svc, m1, m2) = (CString::new("demo.v1.Echo").unwrap(), CString::new("Say").unwrap(), CString::new("Nope").unwrap());
        assert_eq!(rporea_method(b, svc.as_ptr(), m1.as_ptr(), Some(echo), std::ptr::null_mut()), RPOREA_OK);
        assert_eq!(rporea_method(b, svc.as_ptr(), m2.as_ptr(), Some(deny), std::ptr::null_mut()), RPOREA_OK);
        // ヌル・不正を渡しても落ちない
        assert_eq!(rporea_asset(b, std::ptr::null(), ct.as_ptr(), b"x".as_ptr(), 1), RPOREA_E_NULL);
        assert_eq!(rporea_method(b, svc.as_ptr(), m1.as_ptr(), None, std::ptr::null_mut()), RPOREA_E_NULL);
        assert!(rporea_new(std::ptr::null()).is_null());
        let port = rporea_start(b, 0);
        assert!(port > 0, "{port}");
        let r = http(port, "GET /hello.txt HTTP/1.1\r\nhost: x\r\nconnection: close\r\n\r\n");
        assert!(r.starts_with("HTTP/1.1 200") && r.ends_with("hi"), "{r}");
        let body = "{\"a\":1}";
        let r = http(
            port,
            &format!("POST /demo.v1.Echo/Say HTTP/1.1\r\nhost: x\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}", body.len()),
        );
        assert!(r.starts_with("HTTP/1.1 200") && r.ends_with("{\"echo\":{\"a\":1}}"), "{r}");
        let r = http(port, "POST /demo.v1.Echo/Nope HTTP/1.1\r\nhost: x\r\ncontent-length: 2\r\nconnection: close\r\n\r\n{}");
        assert!(r.starts_with("HTTP/1.1 403"), "{r}");
    }

    #[test]
    fn version_is_a_c_string() {
        let v = unsafe { CStr::from_ptr(rporea_version()) }.to_str().unwrap();
        assert!(!v.is_empty());
    }
}
