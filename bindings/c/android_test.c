/* RPoRea C ABI test for Android / Linux: start a server, call it over HTTP from the same process.
 * Build (NDK): aarch64-linux-android24-clang android_test.c -I../../include -L<dir with librporea_ffi.so> -lrporea_ffi -o android_test
 * Run: LD_LIBRARY_PATH=. ./android_test
 */
#include <arpa/inet.h>
#include <netinet/in.h>
#include <stdio.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>
#include "rporea.h"

static intptr_t echo_cb(void *user, const uint8_t *body, size_t len, uint8_t *out, size_t cap) {
    (void)user;
    const char *pre = "{\"echo\":";
    size_t n = strlen(pre);
    if (n + len + 1 > cap) return -500;
    memcpy(out, pre, n);
    memcpy(out + n, body, len);
    out[n + len] = '}';
    return (intptr_t)(n + len + 1);
}

static intptr_t deny_cb(void *user, const uint8_t *body, size_t len, uint8_t *out, size_t cap) {
    (void)user; (void)body; (void)len; (void)out; (void)cap;
    return -403;
}

static int http(int port, const char *req, char *resp, size_t cap) {
    int s = socket(AF_INET, SOCK_STREAM, 0);
    struct sockaddr_in a;
    memset(&a, 0, sizeof a);
    a.sin_family = AF_INET;
    a.sin_port = htons((uint16_t)port);
    inet_pton(AF_INET, "127.0.0.1", &a.sin_addr);
    if (connect(s, (struct sockaddr *)&a, sizeof a) != 0) { close(s); return -1; }
    write(s, req, strlen(req));
    size_t got = 0;
    ssize_t r;
    while (got + 1 < cap && (r = read(s, resp + got, cap - 1 - got)) > 0) got += (size_t)r;
    resp[got] = 0;
    close(s);
    return (int)got;
}

int main(void) {
    printf("version: %s\n", rporea_version());
    RPoReaBuilder *b = rporea_new("android c test");
    rporea_asset(b, "/hello.txt", "text/plain", (const uint8_t *)"hello android", 13);
    rporea_method(b, "demo.v1.Echo", "Say", echo_cb, NULL);
    rporea_method(b, "demo.v1.Echo", "Deny", deny_cb, NULL);
    int port = rporea_start(b, 0);
    printf("port: %d\n", port);
    if (port <= 0) return 2;
    usleep(300000);
    char resp[4096];
    int fails = 0;
    const char *post = "POST /demo.v1.Echo/Say HTTP/1.1\r\nHost: x\r\nContent-Type: application/json\r\nContent-Length: 7\r\nConnection: close\r\n\r\n{\"a\":1}";
    http(port, post, resp, sizeof resp);
    if (!strstr(resp, "200") || !strstr(resp, "{\"echo\":{\"a\":1}}")) { printf("FAIL say: %s\n", resp); fails++; } else puts("ok say");
    http(port, "GET /hello.txt HTTP/1.1\r\nHost: x\r\nConnection: close\r\n\r\n", resp, sizeof resp);
    if (!strstr(resp, "hello android")) { printf("FAIL asset: %s\n", resp); fails++; } else puts("ok asset");
    http(port, "POST /demo.v1.Echo/Deny HTTP/1.1\r\nHost: x\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{}", resp, sizeof resp);
    if (!strstr(resp, " 403")) { printf("FAIL deny: %s\n", resp); fails++; } else puts("ok deny 403");
    http(port, "GET /demo.v1.Echo/Say HTTP/1.1\r\nHost: x\r\nConnection: close\r\n\r\n", resp, sizeof resp);
    if (!strstr(resp, " 405")) { printf("FAIL get405: %s\n", resp); fails++; } else puts("ok GET 405 (no REST)");
    printf(fails ? "FAILED %d\n" : "ALL OK\n", fails);
    return fails ? 1 : 0;
}
