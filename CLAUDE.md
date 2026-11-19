# RPoRea 開発ルール

- **役割**: RPoem+Reaxt の融合。Reaxt のアプリを RPoem 上で起動する入口と、融合の共通部品を置く。個別の実装は Reaxt / RPoem / open-cosmo 側に置き、ここへ複製しない。
- **3姉妹(Reaxt・RPoRea・open-cosmo)は常に同時開発**する。約束・接点・open-cosmo の正式名称の確認結果は [`docs/THREE-SISTERS.md`](docs/THREE-SISTERS.md)。接点を変えたら3つの CLAUDE.md を同じ日に更新する。
- 兄弟ディレクトリ(`../Reaxt` `../RPoem` `../open-cosmo`)が前提。
- `cargo test --workspace` と、実サーバー・実ブラウザでの確認を通してから完了と報告する。確認していないことは「未確認」と書く。
- REST は作らない(Connect のみ)。JSON は RS-JSON。秘密は表示しない。push・タグは確認してから。
- RPoem の `Server::run` は「起動して返す」ので、`JoinHandle` を待たないとプロセスが終わる(`rporea::serve` が待つ)。
- 文書: PC/VPS は日本語、GitHub は最大10言語程度(現状は日英)。

## 次の作業
1. open-cosmo のルーターに Reaxt のサブグラフ(`POST /_reaxt/graphql`)を実際に合成させて確認する(3姉妹の最優先の未了)。
2. 融合の共通部品の洗い出し(Reaxt の v2 設計 `Reaxt/DESIGN-REVIEW.md` と歩調を合わせる)。
3. `rporea` CLI、SET への配備。
