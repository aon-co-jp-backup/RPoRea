# 他の言語から使う(C ABI)

RPoRea は基本 **Rust から**使います(`rporea` crate)。Rust 以外の言語からは、共有ライブラリ `rporea-ffi` の C ABI(`include/rporea.h`)を呼びます。
関数は少なく(作る→静的応答を登録→Connect メソッドを登録→起動)、API は REST を持たず Connect(POST + JSON)のみです。

| 言語 | 状態 | 使い方 |
|---|---|---|
| Rust | 動作・試験あり | `rporea::serve(app, addr)` / `rporea::start` |
| C / C++ | 動作(試験は Rust 側のコールバックで実施、C のサンプルは未作成) | `include/rporea.h` |
| Python | **動作確認済み**(Windows・Linux〈WSL Ubuntu〉、Python 3.14 64bit) | `bindings/python/rporea.py`(標準ライブラリの ctypes のみ)。試験 `bindings/python/test_rporea.py` |
| Mojo | **未検証**(この PC に Mojo が無い) | Mojo の `OwnedDLHandle` で同じ共有ライブラリの関数を呼ぶ想定。下に雛形 |
| RMojo | **未検証** | RMojo は Python 構文なので `rporea.py` をそのまま使える可能性が高いが、確認していない |

## ビルド
```powershell
cargo build -p rporea-ffi --release   # target/release/rporea_ffi.dll (Windows) / librporea_ffi.so (Linux)
py -3 bindings/python/test_rporea.py
```
Python は共有ライブラリを `RPOREA_LIB` 環境変数、または `target/{release,debug}` から探します。

## Python の例
```python
import rporea
app = rporea.App("My site")
app.asset("/", "text/html; charset=utf-8", "<h1>Hello</h1>")
app.method("demo.v1.Echo", "Say", lambda body: '{"ok": true}')   # POST /demo.v1.Echo/Say
app.serve(8080)
```
コールバックが例外を投げると HTTP 500(プロセスは落ちない)、`(403, "msg")` のタプルを返すとそのステータスになります。

## Mojo の雛形(未検証)
```mojo
from sys.ffi import OwnedDLHandle
# lib = OwnedDLHandle("target/release/rporea_ffi.dll")
# new = lib.get_function[fn(UnsafePointer[UInt8]) -> UnsafePointer[NoneType]]("rporea_new")
```
実際の型・コールバックの渡し方は Mojo の FFI の版によって違うので、Mojo を入れた環境で確認してから文書を確定します。

## 対応する OS
- いま: **Windows と Linux**(Windows は実機、Linux は WSL の Ubuntu で確認)。
- リリースが完成してから: **Mac・Android・iPhone・iPad・スーパーコンピューター**(Linux 系の HPC)。それぞれ共有ライブラリ(`.dylib`/`.so`/静的ライブラリ)のビルドと、言語側の結合の確認が要ります。いずれも未着手。
