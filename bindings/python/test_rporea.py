"""Python から RPoRea の共有ライブラリを実際に呼ぶ試験(`python test_rporea.py`)。"""
import http.client
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rporea  # noqa: E402


def request(port, method, path, body=None):
    c = http.client.HTTPConnection("127.0.0.1", port, timeout=10)
    headers = {"content-type": "application/json"} if body is not None else {}
    c.request(method, path, body=body, headers=headers)
    r = c.getresponse()
    data = r.read().decode("utf-8")
    c.close()
    return r.status, data


def main():
    assert rporea.version()
    app = rporea.App("py test")
    app.asset("/hello.txt", "text/plain; charset=utf-8", "こんにちは")
    app.asset("/blob.bin", "application/octet-stream", bytes([0, 1, 2, 255]))
    app.method("demo.v1.Echo", "Say", lambda body: json.dumps({"echo": json.loads(body)}))
    app.method("demo.v1.Echo", "Deny", lambda body: (403, "no"))
    app.method("demo.v1.Echo", "Boom", lambda body: 1 / 0)
    port = app.start(0)
    assert port > 0, port

    st, body = request(port, "GET", "/hello.txt")
    assert (st, body) == (200, "こんにちは"), (st, body)
    st, body = request(port, "POST", "/demo.v1.Echo/Say", json.dumps({"a": [1, 2], "名": "値"}))
    assert st == 200 and json.loads(body) == {"echo": {"a": [1, 2], "名": "値"}}, (st, body)
    st, _ = request(port, "POST", "/demo.v1.Echo/Deny", "{}")
    assert st == 403, st
    st, _ = request(port, "POST", "/demo.v1.Echo/Boom", "{}")
    assert st == 500, st  # Python の例外は 500 になり、プロセスは落ちない
    st, _ = request(port, "GET", "/demo.v1.Echo/Say")
    assert st == 405, st  # Connect は POST のみ(REST 風の GET は受けない)
    st, _ = request(port, "POST", "/demo.v1.Echo/Unknown", "{}")
    assert st == 404, st

    # 消費済みのビルダーは使えない
    try:
        app.start(0)
        raise AssertionError("second start should fail")
    except rporea.RPoReaError:
        pass
    print("python binding OK: port", port)


if __name__ == "__main__":
    main()
