# API 方針(RPoRea) / API policy (RPoRea)

## 日本語
正本は `aon-co-jp/runo` の `API-POLICY.md`。要点: REST は全廃し、新しい REST は作らない。ブラウザや `curl` からは Connect(`POST /<パッケージ>.<Service>/<Method>`、JSON)、サービス間は gRPC / gRPC-web。GraphQL は問い合わせ(読み取り)専用。JSON の読み書きは RS-JSON に統一する。

- **このリポジトリの状況(2026-11-19)**: RPoem と Reaxt を合わせた土台。自前の REST は作らず、Rust 以外からは `rporea-ffi` の C ABI(`include/rporea.h`)を使う。
- **例外(相手の規格が形を決めるもの)**: ヘルスチェック、静的ファイル、第三者の Webhook、ACME、OAuth/OIDC のコールバック、OTLP。
- **3姉妹**: `Reaxt`・`RPoRea`・`open-cosmo` は常に同時に開発する。接点(Cosmo のサブグラフ形式・manifest・Connect・`x-reaxt`)を変えたら、3つの文書を同じ日に更新する。

## English
The source of truth is `API-POLICY.md` in `aon-co-jp/runo`. In short: REST is being abolished and no new REST is added. Browsers and `curl` use Connect (`POST /<package>.<Service>/<Method>`, JSON); service-to-service calls use gRPC / gRPC-web. GraphQL is for read-only queries. JSON is read and written through RS-JSON only.

- **Status of this repository (2026-11-19)**: The foundation that joins RPoem and Reaxt. It builds no REST of its own; non-Rust callers use the C ABI of `rporea-ffi` (`include/rporea.h`).
- **Exceptions (shapes dictated by someone else's standard)**: health checks, static files, third-party webhooks, ACME, OAuth/OIDC callbacks, OTLP.
- **The three sisters**: `Reaxt`, `RPoRea` and `open-cosmo` are always developed together. When an interface between them changes (the Cosmo subgraph format, the manifest, Connect, `x-reaxt`), all three documents are updated on the same day.
