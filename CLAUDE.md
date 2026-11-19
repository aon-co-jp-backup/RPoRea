# RPoRea 開発ルール

- **役割**: RPoem+Reaxt の融合。Reaxt のアプリを RPoem 上で起動する入口(`rporea`)と、他の言語向けの C ABI(`rporea-ffi`)を置く。個別の実装は Reaxt / RPoem / open-cosmo 側に置き、ここへ複製しない。
- **3姉妹(Reaxt・RPoRea・open-cosmo)は常に同時開発**する。約束と接点は [`docs/THREE-SISTERS.md`](docs/THREE-SISTERS.md)。接点を変えたら3つの CLAUDE.md を同じ日に更新する。
- 兄弟ディレクトリ(`../Reaxt` `../RPoem` `../open-cosmo`)が前提。
- `cargo test --workspace` と、実サーバー・実ブラウザ(Python は `bindings/python/test_rporea.py`)での確認を通してから完了と報告する。確認していないことは「未確認」と書く。
- REST は作らない(Connect のみ)。JSON は RS-JSON。秘密は表示しない。push・タグは確認してから。
- RPoem の `Server::run` は「起動して返す」ので、`JoinHandle` を待たないとプロセスが終わる(`rporea::serve` が待つ)。
- 文書(README/CLAUDE/PORTING)は更新のたびに **10 か国語**(日・英・簡中・繁中〈台湾〉・韓・独・仏・西・葡〈ブラジル〉・露)で。ネイティブに近い自然さで、IT・Web・AI の専門家が、子供から大人まで分かるように、短く、やさしい言葉で書く。
- 次の作業: ①(2026-11-19 ゲートウェイ経由の実機確認は済み、`docs/THREE-SISTERS.md`)残りは複数サブグラフにまたがる問い合わせとレジストリ登録、認証ヘッダ転送の判断 ②Mojo/RMojo の検証 ③`.ts` → JS → アセット登録の CLI ④Mac・モバイル。

## 日本語

- 使い方: 基本は Rust から。ほかの言語は `rporea-ffi`(C ABI)から。
- Web の部品: Rust 版 HTML5・CSS3・TypeScript と一緒に使う。HTML/CSS は少なく、tokio と WASM は適材適所、REST なし、TypeScript が先。
- 姉妹(Reaxt・RPoRea・open-cosmo)は、いつも一緒に開発する。
- 文書は更新のたびに 10 か国語で。短く、やさしい言葉で。

## English

- Use: from Rust first. Other languages use `rporea-ffi` (C ABI).
- Web parts: use with Rust HTML5, CSS3 and TypeScript. Little HTML/CSS, tokio and WASM where they fit, no REST, TypeScript first.
- The sisters (Reaxt, RPoRea, open-cosmo) are always developed together.
- Update the docs in 10 languages every time. Keep them short and simple.

## 简体中文

- 用法:先用 Rust。其他语言通过 `rporea-ffi`(C ABI)。
- Web 部件:与 Rust 版 HTML5、CSS3、TypeScript 一起用。少用 HTML/CSS,tokio 和 WASM 因地制宜,不用 REST,先做 TypeScript。
- 姐妹项目(Reaxt、RPoRea、open-cosmo)始终一起开发。
- 每次更新文档,都要有 10 种语言版本。写得简短、通俗。

## 繁體中文(台灣)

- 用法:先用 Rust。其他語言透過 `rporea-ffi`(C ABI)。
- Web 元件:與 Rust 版 HTML5、CSS3、TypeScript 一起用。少用 HTML/CSS,tokio 和 WASM 因地制宜,不用 REST,先做 TypeScript。
- 姊妹專案(Reaxt、RPoRea、open-cosmo)始終一起開發。
- 每次更新文件,都要有 10 種語言版本。寫得簡短、易懂。

## 한국어

- 사용법: 먼저 Rust로. 다른 언어는 `rporea-ffi`(C ABI)로.
- 웹 부품: Rust판 HTML5, CSS3, TypeScript와 함께 사용. HTML/CSS는 적게, tokio와 WASM은 알맞게, REST 없음, TypeScript 먼저.
- 자매 프로젝트(Reaxt, RPoRea, open-cosmo)는 항상 함께 개발합니다.
- 문서는 고칠 때마다 10개 언어로. 짧고 쉬운 말로.

## Deutsch

- Nutzung: zuerst aus Rust. Andere Sprachen über `rporea-ffi` (C-ABI).
- Web-Teile: zusammen mit Rust-HTML5, -CSS3 und -TypeScript. Wenig HTML/CSS, tokio und WASM wo passend, kein REST, TypeScript zuerst.
- Die Schwesterprojekte (Reaxt, RPoRea, open-cosmo) werden immer gemeinsam entwickelt.
- Die Dokumente bei jeder Änderung in 10 Sprachen. Kurz und einfach.

## Français

- Usage : d'abord depuis Rust. Les autres langages passent par `rporea-ffi` (ABI C).
- Briques web : avec HTML5, CSS3 et TypeScript en Rust. Peu de HTML/CSS, tokio et WASM là où ils conviennent, pas de REST, TypeScript d'abord.
- Les projets sœurs (Reaxt, RPoRea, open-cosmo) avancent toujours ensemble.
- Documents en 10 langues à chaque mise à jour. Courts et simples.

## Español

- Uso: primero desde Rust. Los demás lenguajes usan `rporea-ffi` (ABI de C).
- Piezas web: junto con HTML5, CSS3 y TypeScript en Rust. Poco HTML/CSS, tokio y WASM donde encajen, sin REST, TypeScript primero.
- Los proyectos hermanos (Reaxt, RPoRea, open-cosmo) se desarrollan siempre juntos.
- Los documentos, en 10 idiomas en cada cambio. Cortos y sencillos.

## Português (Brasil)

- Uso: primeiro pelo Rust. As outras linguagens usam o `rporea-ffi` (ABI C).
- Peças web: junto com HTML5, CSS3 e TypeScript em Rust. Pouco HTML/CSS, tokio e WASM onde couberem, sem REST, TypeScript primeiro.
- Os projetos irmãos (Reaxt, RPoRea, open-cosmo) são sempre feitos juntos.
- Documentos em 10 idiomas a cada mudança. Curtos e simples.

## Русский

- Использование: сначала из Rust. Другие языки — через `rporea-ffi` (C ABI).
- Веб-части: вместе с HTML5, CSS3 и TypeScript на Rust. Мало HTML/CSS, tokio и WASM — где уместно, без REST, сначала TypeScript.
- Сёстры (Reaxt, RPoRea, open-cosmo) всегда развиваются вместе.
- Документы при каждом изменении — на 10 языках. Коротко и просто.
