/* RPoRea の C ABI(rporea-ffi)。Python・Mojo・RMojo・C/C++ などから同じ共有ライブラリを呼ぶ。
 * Windows: rporea_ffi.dll / Linux: librporea_ffi.so / macOS: librporea_ffi.dylib(Mac は今後)。
 *
 * 約束:
 *  - 関数は NULL や不正な UTF-8 を渡されても落ちず、負の値(エラー)を返す。
 *  - rporea_start / rporea_serve は Builder を消費する(その後は使えない)。
 *  - コールバックはサーバーのワーカースレッドから呼ばれる。スレッド安全性は呼び出し側の責任。
 *  - REST は提供しない。メソッドは Connect(POST /pkg.Service/Method + JSON)として公開される。
 */
#ifndef RPOREA_H
#define RPOREA_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct RPoReaBuilder RPoReaBuilder;

#define RPOREA_OK 0
#define RPOREA_E_NULL (-1)
#define RPOREA_E_UTF8 (-2)
#define RPOREA_E_CONSUMED (-3)
#define RPOREA_E_START (-4)

/* body(JSON, UTF-8)を受け、out(容量 out_cap)へ応答を書いて、書いた長さを返す。
 * 負の値は失敗(-403 なら HTTP 403。400〜599 に丸める)。out_cap は最大 1 MiB。 */
typedef intptr_t (*rporea_method_cb)(void *user, const uint8_t *body, size_t body_len, uint8_t *out, size_t out_cap);

const char *rporea_version(void);
RPoReaBuilder *rporea_new(const char *title);
void rporea_free(RPoReaBuilder *b);
int rporea_asset(RPoReaBuilder *b, const char *path, const char *content_type, const uint8_t *data, size_t len);
int rporea_method(RPoReaBuilder *b, const char *service, const char *method, rporea_method_cb cb, void *user);
/* バックグラウンドで起動。実際のポート(正)を返す。port=0 なら空きポート。失敗は負。 */
int rporea_start(RPoReaBuilder *b, uint16_t port);
/* 起動して、サーバーが終わるまで戻らない。 */
int rporea_serve(RPoReaBuilder *b, uint16_t port);

#ifdef __cplusplus
}
#endif
#endif
