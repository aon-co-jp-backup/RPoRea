"""Mojo から RPoRea の C ABI を実際に呼ぶ試験 / Calls RPoRea's C ABI from Mojo.

Run (WSL, Mojo 1.2.0):
    RPOREA_LIB=/path/to/librporea_ffi.so mojo test_rporea.mojo
"""
from std.ffi import OwnedDLHandle
from std.os import getenv
from std.python import Python


def main() raises:
    var path = getenv("RPOREA_LIB", "librporea_ffi.so")
    var lib = OwnedDLHandle(path)

    # バージョン文字列(C の const char*)
    var ver = lib.call["rporea_version", Int]()
    print("version ptr non-null:", ver != 0)

    # ビルダーを作り、静的ファイルを1つ登録して起動(コールバック無しの最小経路)
    var title = String("mojo test")
    var b = lib.call["rporea_new", Int](Int(title.unsafe_ptr()))
    var body = String("hello from mojo")
    var path_a = String("/hello.txt")
    var path_b = String("text/plain")
    var rc = lib.call["rporea_asset", Int32](
        b,
        Int(path_a.unsafe_ptr()),
        Int(path_b.unsafe_ptr()),
        Int(body.unsafe_ptr()),
        UInt(body.byte_length()),
    )
    print("asset rc:", rc)
    var port = lib.call["rporea_start", Int32](b, UInt16(0))
    print("port:", port)
    if port <= 0:
        raise Error("rporea_start failed")

    # 実際にHTTPで取りに行く(Python の標準ライブラリを使う)
    var http = Python.import_module("http.client")
    var c = http.HTTPConnection("127.0.0.1", Int(port), timeout=10)
    c.request("GET", "/hello.txt")
    var r = c.getresponse()
    var text = String(py=r.read().decode("utf-8"))
    print("status:", r.status, "body:", text)
    if Int(py=r.status) != 200 or text != "hello from mojo":
        raise Error("unexpected response")
    # 登録していないメソッドは 404(コールバック付きのメソッドは別途: Mojo から C の関数ポインタを渡す部分は未了)
    c.request("GET", "/demo.v1.Echo/Say")
    var r2 = c.getresponse()
    r2.read()
    print("GET on an unregistered method:", r2.status)
    if Int(py=r2.status) != 404:
        raise Error("expected 404")
    print("ALL OK")
