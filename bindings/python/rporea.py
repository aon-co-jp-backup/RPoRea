"""RPoRea の Python 向け結合(標準ライブラリの ctypes だけ。追加の依存なし)。

    import rporea
    app = rporea.App("My site")
    app.asset("/", "text/html; charset=utf-8", "<h1>Hello</h1>")
    app.method("demo.v1.Echo", "Say", lambda body: '{"echo": %s}' % body)   # Connect(POST /demo.v1.Echo/Say)
    port = app.start(0)     # 空きポートで起動して、実際のポートを返す(バックグラウンド)
    # app.serve(8080)       # こちらは終わるまで戻らない

REST は使わない方針: メソッドは Connect(POST + JSON)として公開される。
共有ライブラリは、環境変数 RPOREA_LIB、または同じ場所・`target/{debug,release}` から探す。
"""
import ctypes
import os
import sys

__all__ = ["App", "RPoReaError", "version"]

_OUT_CAP = 1 << 20
_CALLBACK = ctypes.CFUNCTYPE(
    ctypes.c_ssize_t, ctypes.c_void_p, ctypes.POINTER(ctypes.c_ubyte), ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_ubyte), ctypes.c_size_t,
)

_ERRORS = {-1: "null argument", -2: "invalid UTF-8", -3: "builder already consumed", -4: "failed to start"}


class RPoReaError(RuntimeError):
    pass


def _lib_name():
    if sys.platform.startswith("win"):
        return "rporea_ffi.dll"
    if sys.platform == "darwin":
        return "librporea_ffi.dylib"
    return "librporea_ffi.so"


def _find_lib():
    env = os.environ.get("RPOREA_LIB")
    if env:
        return env
    here = os.path.dirname(os.path.abspath(__file__))
    root = os.path.abspath(os.path.join(here, "..", ".."))
    for d in (here, os.path.join(root, "target", "release"), os.path.join(root, "target", "debug")):
        p = os.path.join(d, _lib_name())
        if os.path.exists(p):
            return p
    raise RPoReaError("%s が見つかりません。`cargo build -p rporea-ffi` するか RPOREA_LIB を設定してください" % _lib_name())


_lib = None


def _load():
    global _lib
    if _lib is not None:
        return _lib
    lib = ctypes.CDLL(_find_lib())
    lib.rporea_version.restype = ctypes.c_char_p
    lib.rporea_new.restype = ctypes.c_void_p
    lib.rporea_new.argtypes = [ctypes.c_char_p]
    lib.rporea_free.argtypes = [ctypes.c_void_p]
    lib.rporea_asset.restype = ctypes.c_int
    lib.rporea_asset.argtypes = [ctypes.c_void_p, ctypes.c_char_p, ctypes.c_char_p, ctypes.c_char_p, ctypes.c_size_t]
    lib.rporea_method.restype = ctypes.c_int
    lib.rporea_method.argtypes = [ctypes.c_void_p, ctypes.c_char_p, ctypes.c_char_p, _CALLBACK, ctypes.c_void_p]
    lib.rporea_start.restype = ctypes.c_int
    lib.rporea_start.argtypes = [ctypes.c_void_p, ctypes.c_uint16]
    lib.rporea_serve.restype = ctypes.c_int
    lib.rporea_serve.argtypes = [ctypes.c_void_p, ctypes.c_uint16]
    _lib = lib
    return lib


def _check(code):
    if code < 0:
        raise RPoReaError(_ERRORS.get(code, "error %d" % code))
    return code


def version():
    return _load().rporea_version().decode("ascii")


class App:
    def __init__(self, title):
        self._lib = _load()
        self._handle = self._lib.rporea_new(title.encode("utf-8"))
        if not self._handle:
            raise RPoReaError("could not create the app")
        self._callbacks = []  # コールバックをサーバーが動いている間ずっと生かしておく(GC対策)

    def asset(self, path, content_type, data):
        """`GET path` で静的な応答(文字列またはバイト列)を返す。"""
        if isinstance(data, str):
            data = data.encode("utf-8")
        _check(self._lib.rporea_asset(self._handle, path.encode("utf-8"), content_type.encode("utf-8"), data, len(data)))
        return self

    def method(self, service, name, fn):
        """Connect メソッド(POST /service/name)。`fn(body: str) -> str(JSON)`。
        例外を投げると HTTP 500、`rporea.Denied` 相当は (status, message) を返す代わりに `raise RPoReaError(status)` ではなく
        `return (403, "msg")` のタプルで表す。"""
        def trampoline(_user, body, body_len, out, out_cap):
            try:
                text = ctypes.string_at(body, body_len).decode("utf-8")
                r = fn(text)
                if isinstance(r, tuple):
                    return -int(r[0])
                data = r.encode("utf-8")
                if len(data) > out_cap:
                    return -500
                ctypes.memmove(out, data, len(data))
                return len(data)
            except Exception:  # 例外を C の境界の外へ出さない
                return -500

        cb = _CALLBACK(trampoline)
        self._callbacks.append(cb)
        _check(self._lib.rporea_method(self._handle, service.encode("utf-8"), name.encode("utf-8"), cb, None))
        return self

    def _take(self):
        h, self._handle = self._handle, None
        if not h:
            raise RPoReaError("builder already consumed")
        return h

    def start(self, port=0):
        """バックグラウンドで起動し、実際のポートを返す。"""
        return _check(self._lib.rporea_start(self._take(), port))

    def serve(self, port=8080):
        """起動して、終わるまで戻らない。"""
        _check(self._lib.rporea_serve(self._take(), port))

    def __del__(self):
        h = getattr(self, "_handle", None)
        if h:
            self._lib.rporea_free(h)
            self._handle = None
