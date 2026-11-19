# RPoRea / RPoem と Rust 版 HTML5+CSS3+TypeScript(2026-11-19、ユーザー指示)

**前提**: RPoRea と RPoem は基本 **Rust から**使い、Web サイト・Web アプリ・アプリを作ります。開発中の **HTML5+CSS3+TypeScript の Rust 版**と一緒に使う想定です。

## 方針
1. **HTML5 / CSS3 の使用量は極力少なく**する。画面は Reaxt の部品(VNode)から組み、素の HTML/CSS は最小(外枠・必要なスタイルだけ)。
2. **tokio と WASM は適材適所**。サーバー側の非同期は tokio、ブラウザ内の動きは WASM。全部を WASM にもしない。
3. **REST は使わない**。ブラウザ/`curl` からは Connect(POST + JSON)、サービス間は gRPC / gRPC-web。
4. **JavaScript の Rust 版も進めるが、TypeScript の Rust 版を優先**する(ユーザー指示)。

## 部品(すべて `aon-co-jp`)と現状(2026-11-19 調査)
| リポジトリ | 役割 | 状態 |
|---|---|---|
| [`rhtml5`](https://github.com/aon-co-jp/rhtml5) | HTML5 のパーサー/DOM | トークナイザのみ(約 775 行)。DOM 木・レイアウトは未着手 |
| [`rcss3`](https://github.com/aon-co-jp/rcss3) | CSS3 のパーサー/カスケード/スタイル計算 | パーサー+カスケード(約 440 行) |
| [`RS-TypeScript`](https://github.com/aon-co-jp/RS-TypeScript) | TypeScript → JavaScript(型の除去と実行時構文の展開) | **本日拡張**(`import type`・`declare`・`namespace`・`implements`・`abstract`・複数宣言子・`satisfies` など、Node で実行確認)。型チェックは対象外 |
| [`RS-JavaScript`](https://github.com/aon-co-jp/RS-JavaScript) | JavaScript の構文解析・実行エンジン | 雛形のみ(11 行) |
| [`RS-HTML`](https://github.com/aon-co-jp/RS-HTML) / [`RS-CSS`](https://github.com/aon-co-jp/RS-CSS) | HTML/CSS の Rust+Poem(+WASM)版 | 既存(小規模) |
| [`RS-JSON`](https://github.com/aon-co-jp/RS-JSON) | JSON(安全+高速) | 既存 |

`RS-React` は廃止済み(Reaxt に統合)。

## つなぎ方の想定(未実装)
- **ビルド時**: `.ts` を `RS-TypeScript` で JS にし、`rhtml5`/`rcss3` で HTML/CSS を検査・最小化して、Reaxt のアセットとして `rporea_asset` / `App::asset` に載せる。
- **サーバー側**: Reaxt の SSR(VNode → HTML)+ RPoem(tokio)。
- **ブラウザ側**: Reaxt の WASM(VNode のハイドレーション)。素の JS/TS は、WASM で書けない部分(DOM の薄い橋渡し)だけ。
- 次に作る具体物の候補: (1) `RS-TypeScript` のソースマップ・Wasm 向け出力、(2) `rhtml5` の DOM 木、(3) RPoRea のビルド手順(`.ts` → JS → アセット登録)を自動化する CLI。いずれも未着手。
