"""Mojo の関数を C のコールバックとして RPoRea に渡す試験 / Passes a Mojo function as a C callback."""
from std.ffi import OwnedDLHandle
from std.os import getenv
from std.python import Python


def echo_cb(user: Int, body: Int, blen: UInt, dst: Int, cap: UInt) abi("C") -> Int:
    # {"echo": <body>} を out へ書く
    var pre = String('{"echo":')
    var n = pre.byte_length()
    if n + Int(blen) + 1 > Int(cap):
        return -500
    var outp = UnsafePointer[UInt8, MutAnyOrigin](unsafe_from_address=dst)
    var bodyp = UnsafePointer[UInt8, MutAnyOrigin](unsafe_from_address=body)
    for i in range(n):
        outp[i] = pre.unsafe_ptr()[i]
    for i in range(Int(blen)):
        outp[n + i] = bodyp[i]
    outp[n + Int(blen)] = UInt8(ord("}"))
    return n + Int(blen) + 1


def main() raises:
    var lib = OwnedDLHandle(getenv("RPOREA_LIB", "librporea_ffi.so"))
    var title = String("mojo cb")
    var b = lib.call["rporea_new", Int](Int(title.unsafe_ptr()))
    var svc = String("demo.v1.Echo")
    var mth = String("Say")
    var rc = lib.call["rporea_method", Int32](b, Int(svc.unsafe_ptr()), Int(mth.unsafe_ptr()), echo_cb, Int(0))
    print("method rc:", rc)
    var port = lib.call["rporea_start", Int32](b, UInt16(0))
    var http = Python.import_module("http.client")
    var c = http.HTTPConnection("127.0.0.1", Int(port), timeout=10)
    c.request("POST", "/demo.v1.Echo/Say", '{"a":1}', {"content-type": "application/json"})
    var r = c.getresponse()
    var text = String(py=r.read().decode("utf-8"))
    print("status:", r.status, "body:", text)
    if Int(py=r.status) != 200 or text != '{"echo":{"a":1}}':
        raise Error("unexpected response")
    print("ALL OK")
