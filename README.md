# RPoRea

**RPoem + Reaxt の融合。** Reaxt(Rust だけで書くフロント+バックエンドの枠組み)のアプリを、RPoem(Poem+Tauri 互換のサーバー)の上に 1 行で載せて起動します。

```rust
use rporea::reaxt::{app::App, react::VNode, RenderMode};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let app = Arc::new(
        App::new("hello")
            .page("index.rs", RenderMode::Ssr, |_| VNode::element("h1").child(VNode::text("Hello")).build())
            .finish(),
    );
    rporea::serve(app, ([127, 0, 0, 1], 8768)).await.unwrap();
}
```

```bash
cargo run -p rporea --example hello
```

`serve` は RPoem のトレースと圧縮を重ねた本番向けの既定(`reaxt_rpoem::route_production`)で起動します。アドレスとハンドルが欲しい場合(試験・停止制御)は `rporea::start` を使います。

## 3姉妹リポジトリ(常に同時開発)
[`Reaxt`](https://github.com/aon-co-jp/Reaxt) / **RPoRea** / [`open-cosmo`](https://github.com/aon-co-jp/open-cosmo)(WunderGraph Cosmo 相当の Rust 製オープンソース版)。
約束と接点の現状は [`docs/THREE-SISTERS.md`](docs/THREE-SISTERS.md)。3 つと親の [`RPoem`](https://github.com/aon-co-jp/RPoem) は兄弟ディレクトリに置きます(パス依存)。

## 現状(正直に)
- 動くもの: Reaxt のアプリを RPoem 上で起動する入口(`start`/`serve`)、試験 1 件(実 TCP で 200 とページ内容を確認)。
- 未着手: open-cosmo のルーターとの合成(Reaxt のサブグラフを実際に束ねる確認)、融合のための共通部品、CLI、配備(SET)。
- ライセンス: Apache-2.0 OR MIT

---

# RPoRea (English)

**A fusion of RPoem and Reaxt.** Run a Reaxt app (a Rust-only full-stack framework) on RPoem (a Poem/Tauri-compatible server) with one call: `rporea::serve(app, addr)`. It uses the production defaults of `reaxt_rpoem::route_production` (tracing and compression); use `rporea::start` when you need the bound address and the task handle.

**Three sister repositories, always developed together**: [`Reaxt`](https://github.com/aon-co-jp/Reaxt), **RPoRea** and [`open-cosmo`](https://github.com/aon-co-jp/open-cosmo) (the Rust open-source counterpart of WunderGraph Cosmo). See [`docs/THREE-SISTERS.md`](docs/THREE-SISTERS.md) for the rules and the current state of the touch points. Keep all three, plus the parent [`RPoem`](https://github.com/aon-co-jp/RPoem), as sibling directories (path dependencies).

**Status**: working — the entry point and one test over real TCP. Not started — composing with the open-cosmo router, shared fusion components, a CLI, deployment on SET. License: Apache-2.0 OR MIT.
