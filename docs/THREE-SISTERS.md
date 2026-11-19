# 3姉妹リポジトリ: Reaxt / RPoRea / open-cosmo(常に同時開発)

| リポジトリ | 役割 | 場所 |
|---|---|---|
| [`Reaxt`](https://github.com/aon-co-jp/Reaxt) | Rustだけで書くフロント+バックエンドの枠組み(部品・SSR/CSR/Islands・Server Actions・DB) | `F:\Reaxt` |
| **`RPoRea`**(本リポジトリ) | RPoem+Reaxtの融合。Reaxtのアプリを1行でRPoem上に載せて起動する入口(`rporea::serve`)。融合に必要な共通部品の置き場 | `F:\RPoRea` |
| [`open-cosmo`](https://github.com/aon-co-jp/open-cosmo) | WunderGraph Cosmo(有料版相当)のRust製オープンソース版(GraphQL Federation、ルーター、スキーマレジストリ等) | `F:\open-cosmo` |

親の [`RPoem`](https://github.com/aon-co-jp/RPoem)(Poem+Tauri互換のサーバー)は、3姉妹がそろって依存する土台。

## open-cosmo の正式名称(確認済み・2026-11-19)
GitHub上の正式なリポジトリ名は **`open-cosmo`**(`aon-co-jp/open-cosmo`、説明「WunderGraph Cosmoの有料版を含む全ての機能の完全無料のオープンソース版」)。
`RCsomo`(綴りの誤り)・`open-runo`・`poem-tauri-cosmo` は旧名/別目的の古いリポジトリで、3姉妹の対象ではない。

## 同時開発の約束
1. **どれか1つを変えたら、他の2つへの影響を必ず確認する**(接点: Reaxtの`gateway`/サブグラフ → open-cosmoのルーター、`reaxt-rpoem` → RPoem、`rporea::serve` → 両方)。
2. 接点の仕様(Cosmoサブグラフ形式、`/_reaxt/manifest.json`、Connectの呼び出し形、ヘッダ`x-reaxt`)を変えるときは、**3つのCLAUDE.mdを同じ日に更新**する(乖離に気づいた側が他へ反映)。
3. 3つとも兄弟ディレクトリに置く(`../Reaxt` `../RPoem` `../open-cosmo`)。パス依存のため。
4. 3つの全体テスト(`cargo test --workspace`)が通ることを、変更のたびに確認する。1つでも赤なら、それを直してから次へ進む。
5. 動作確認は実サーバー・実ブラウザで行い、確認していないことは「未確認」と書く。

## 接点の現状(2026-11-19)
- Reaxt → open-cosmo: Reaxtは `reaxt_actions::subgraph::install(app)` でFederation v2のサブグラフ(`POST /_reaxt/graphql`、`_service{sdl}`)を公開する。**2026-11-19に実機で確認**: 実際のReaxtサーバーを `OPEN_RUNO_SUBGRAPHS` に登録したopen-cosmoのゲートウェイ経由で、問い合わせ `{ reaxt_ready }`=true、ミューテーション `demo_v1_Counter_Inc`=1、`x-reaxt` 無しは403のまま。確認で見つけた2つのずれを直した: (1) Reaxtが `__schema` のイントロスペクションに答えず、ゲートウェイがフィールドを見つけられなかった → Reaxt側で最小応答を追加。(2) ゲートウェイが `x-reaxt` を転送せずCSRFガードで拒否された → ゲートウェイが `x-reaxt` だけを転送(認証系ヘッダは転送しない。転送の是非は未決)。**2026-11-19追加**: 読み取り専用の問い合わせで、ルートフィールドが複数のサブグラフにまたがる場合は、サブグラフごとの部分問い合わせに分けて同時実行し、`data`/`errors`を統合する(ゲートウェイ43件の試験成功)。変更系(mutation)・リモートとローカルの混在・フラグメント/ディレクティブつきは従来どおり拒否。**2026-11-19追加**: `@key` によるエンティティ結合(1段)も実装(サブグラフAのidでBの`_entities`へ問い合わせて合成。擬似サブグラフで試験成功)。**未了**: 複数段の結合・`@requires`/`@provides`・実Reaxtを複数並べた通し。スキーマレジストリ(`open-runo-federation::compose`)へのReaxtのSDL投入は、2026-11-19に実サーバーのSDLで`parse_service_sdl`→`compose`が通ることを確認(open-cosmoの試験`reaxt_subgraph_sdl_composes_with_other_subgraphs`)。管理RPC `ComposeFederation`(sdl経路)でも登録できることを試験で確認(`federation_compose_accepts_reaxt_sdl`)。残りは複数サブグラフをまたぐ問い合わせ計画と`@key`結合。
- Reaxt → RPoem: `reaxt-rpoem`(`route`/`route_production`、ゲートウェイ・manifest・ストリーミング、非同期ストリーミングは実HTTP/実ブラウザで確認済み)。
- RPoRea: `rporea::start`/`serve` が上の2つを束ねる(試験1件)。
