# RPoRea 開発ルール

- **役割**: RPoem+Reaxt の融合。Reaxt のアプリを RPoem 上で起動する入口(`rporea`)と、他の言語向けの C ABI(`rporea-ffi`)を置く。個別の実装は Reaxt / RPoem / open-cosmo 側に置き、ここへ複製しない。
- **3姉妹(Reaxt・RPoRea・open-cosmo)は常に同時開発**する。約束と接点は [`docs/THREE-SISTERS.md`](docs/THREE-SISTERS.md)。接点を変えたら3つの CLAUDE.md を同じ日に更新する。
- 兄弟ディレクトリ(`../Reaxt` `../RPoem` `../open-cosmo`)が前提。
- `cargo test --workspace` と、実サーバー・実ブラウザ(Python は `bindings/python/test_rporea.py`)での確認を通してから完了と報告する。確認していないことは「未確認」と書く。
- REST は作らない(Connect のみ)。JSON は RS-JSON。秘密は表示しない。push・タグは確認してから。
- RPoem の `Server::run` は「起動して返す」ので、`JoinHandle` を待たないとプロセスが終わる(`rporea::serve` が待つ)。
- 文書(README/CLAUDE/PORTING)は更新のたびに **10 か国語**(日・英・簡中・繁中〈台湾〉・韓・独・仏・西・葡〈ブラジル〉・露)で。ネイティブに近い自然さで、IT・Web・AI の専門家が、子供から大人まで分かるように、短く、やさしい言葉で書く。
- 次の作業: ①open-cosmo のルーターに Reaxt のサブグラフ(`POST /_reaxt/graphql`)を実合成して確認(最優先) ②Mojo/RMojo の検証 ③`.ts` → JS → アセット登録の CLI ④Mac・モバイル。

## 日本語

**完成イメージ**: サーバーに RPoRea を**1か所だけ**入れます。RPoRea は上位版で、Reaxt の役目も兼ねます(`easy-web.tokyo/RPoRea`)。これで open-easy-web・RPoem・aruaru-llm・open-directx・open-cuda・open-cpu などを、サイトごとに入れ直す必要がなくなります。各サイトは「分身」として動きます(非同期・マルチスレッド)。省メモリで、速く、安全に。**REST API と Node.js は使いません。**

**現状(正直に)**: できている=RPoRea の入口と Python 連携、認証ヘッダの条件付き転送、分身ホスティングの土台(open-web-server と RPoem で、実プロセスの起動と振り分けを確認)、aruaru-llm の同時実行制限(本番で稼働)。まだ=1か所への入れ替え配備、サイトごとの分離(別ユーザー・メモリ制限)、open-cuda・open-directx・open-cpu を共有基盤として呼ぶ統合、公開ページ。上の完成イメージは目標で、全部できたわけではありません。

- 使い方: 基本は Rust から。ほかの言語は `rporea-ffi`(C ABI)から。
- Web の部品: Rust 版 HTML5・CSS3・TypeScript と一緒に使う。HTML/CSS は少なく、tokio と WASM は適材適所、REST なし、TypeScript が先。
- 姉妹(Reaxt・RPoRea・open-cosmo)は、いつも一緒に開発する。
- 文書は更新のたびに 10 か国語で。短く、やさしい言葉で。

## English

**The goal**: install RPoRea **once** on a server. RPoRea is the upper edition and also does Reaxt's job (`easy-web.tokyo/RPoRea`). Then open-easy-web, RPoem, aruaru-llm, open-directx, open-cuda, open-cpu and the rest do not have to be installed again for every site. Each site runs as a "shadow clone" (async, multi-threaded): light on memory, fast and safe. **No REST API and no Node.js.**

**Where we are (honestly)**: done: the RPoRea entry point and Python support, conditional forwarding of credentials, the base of shadow-clone hosting (real processes started and routed in open-web-server and RPoem), aruaru-llm's concurrency limit (live in production). Not yet: one-place install and deployment, per-site isolation (separate users, memory limits), calling open-cuda, open-directx and open-cpu as shared services, the public pages. The goal above is a target, not something already finished.

- Use: from Rust first. Other languages use `rporea-ffi` (C ABI).
- Web parts: use with Rust HTML5, CSS3 and TypeScript. Little HTML/CSS, tokio and WASM where they fit, no REST, TypeScript first.
- The sisters (Reaxt, RPoRea, open-cosmo) are always developed together.
- Update the docs in 10 languages every time. Keep them short and simple.

## 简体中文

**完成后的样子**:在服务器上**只装一次** RPoRea。RPoRea 是上位版,也承担 Reaxt 的作用(`easy-web.tokyo/RPoRea`)。这样 open-easy-web、RPoem、aruaru-llm、open-directx、open-cuda、open-cpu 等,就不用给每个网站各装一遍。每个网站都以“分身”方式运行(异步、多线程):省内存、速度快、更安全。**不使用 REST API,也不使用 Node.js。**

**现状(如实说明)**:已完成:RPoRea 的入口和 Python 支持、认证头的有条件转发、分身托管的基础(在 open-web-server 和 RPoem 中验证了真实进程的启动和分流)、aruaru-llm 的并发限制(已在线上运行)。尚未完成:一处安装与部署、按网站隔离(独立用户、内存限制)、把 open-cuda、open-directx、open-cpu 作为共享服务来调用、公开页面。上面的样子是目标,并非全部已经做好。

- 用法:先用 Rust。其他语言通过 `rporea-ffi`(C ABI)。
- Web 部件:与 Rust 版 HTML5、CSS3、TypeScript 一起用。少用 HTML/CSS,tokio 和 WASM 因地制宜,不用 REST,先做 TypeScript。
- 姐妹项目(Reaxt、RPoRea、open-cosmo)始终一起开发。
- 每次更新文档,都要有 10 种语言版本。写得简短、通俗。

## 繁體中文(台灣)

**完成後的樣子**:在伺服器上**只裝一次** RPoRea。RPoRea 是上位版,也承擔 Reaxt 的作用(`easy-web.tokyo/RPoRea`)。這樣 open-easy-web、RPoem、aruaru-llm、open-directx、open-cuda、open-cpu 等,就不用替每個網站各裝一遍。每個網站都以「分身」方式執行(非同步、多執行緒):省記憶體、速度快、更安全。**不使用 REST API,也不使用 Node.js。**

**現況(如實說明)**:已完成:RPoRea 的入口與 Python 支援、認證標頭的有條件轉送、分身代管的基礎(在 open-web-server 與 RPoem 中驗證了真實行程的啟動與分流)、aruaru-llm 的並行上限(已在正式環境運作)。尚未完成:一處安裝與部署、依網站隔離(獨立使用者、記憶體限制)、把 open-cuda、open-directx、open-cpu 當作共用服務來呼叫、公開頁面。上面的樣子是目標,並非全都已經做好。

- 用法:先用 Rust。其他語言透過 `rporea-ffi`(C ABI)。
- Web 元件:與 Rust 版 HTML5、CSS3、TypeScript 一起用。少用 HTML/CSS,tokio 和 WASM 因地制宜,不用 REST,先做 TypeScript。
- 姊妹專案(Reaxt、RPoRea、open-cosmo)始終一起開發。
- 每次更新文件,都要有 10 種語言版本。寫得簡短、易懂。

## 한국어

**완성된 모습**: 서버에 RPoRea를 **한 곳에만** 설치합니다. RPoRea는 상위 버전이며 Reaxt의 역할도 함께 합니다(`easy-web.tokyo/RPoRea`). 그러면 open-easy-web, RPoem, aruaru-llm, open-directx, open-cuda, open-cpu 등을 사이트마다 다시 설치할 필요가 없습니다. 각 사이트는 "분신"처럼 동작합니다(비동기, 멀티스레드). 메모리는 적게, 속도는 빠르게, 더 안전하게. **REST API와 Node.js는 쓰지 않습니다.**

**현재 상태(솔직하게)**: 된 것: RPoRea 진입점과 Python 연동, 인증 헤더의 조건부 전달, 분신 호스팅의 기초(open-web-server와 RPoem에서 실제 프로세스 시작과 분배를 확인), aruaru-llm의 동시 실행 제한(운영 중). 아직인 것: 한 곳 설치·배포, 사이트별 분리(별도 사용자, 메모리 제한), open-cuda·open-directx·open-cpu를 공유 서비스로 호출하는 통합, 공개 페이지. 위의 모습은 목표이며 전부 끝난 것은 아닙니다.

- 사용법: 먼저 Rust로. 다른 언어는 `rporea-ffi`(C ABI)로.
- 웹 부품: Rust판 HTML5, CSS3, TypeScript와 함께 사용. HTML/CSS는 적게, tokio와 WASM은 알맞게, REST 없음, TypeScript 먼저.
- 자매 프로젝트(Reaxt, RPoRea, open-cosmo)는 항상 함께 개발합니다.
- 문서는 고칠 때마다 10개 언어로. 짧고 쉬운 말로.

## Deutsch

**Das Ziel**: RPoRea wird **nur einmal** auf dem Server installiert. RPoRea ist die höhere Ausgabe und übernimmt auch die Aufgabe von Reaxt (`easy-web.tokyo/RPoRea`). Dann müssen open-easy-web, RPoem, aruaru-llm, open-directx, open-cuda, open-cpu und die übrigen nicht für jede Website neu installiert werden. Jede Website läuft als „Schattendoppelgänger“ (asynchron, mehrere Threads): sparsam beim Speicher, schnell und sicher. **Keine REST-API und kein Node.js.**

**Wo wir stehen (ehrlich)**: Fertig: der Einstieg von RPoRea und die Python-Anbindung, die bedingte Weitergabe von Anmeldedaten, die Grundlage für Schattendoppelgänger-Hosting (echte Prozesse in open-web-server und RPoem gestartet und verteilt), das Gleichzeitigkeitslimit von aruaru-llm (produktiv). Noch offen: Installation und Auslieferung an einer Stelle, Trennung pro Website (eigene Benutzer, Speicherlimits), Aufruf von open-cuda, open-directx und open-cpu als gemeinsame Dienste, die öffentlichen Seiten. Das Ziel oben ist ein Vorhaben, nicht etwas schon Fertiges.

- Nutzung: zuerst aus Rust. Andere Sprachen über `rporea-ffi` (C-ABI).
- Web-Teile: zusammen mit Rust-HTML5, -CSS3 und -TypeScript. Wenig HTML/CSS, tokio und WASM wo passend, kein REST, TypeScript zuerst.
- Die Schwesterprojekte (Reaxt, RPoRea, open-cosmo) werden immer gemeinsam entwickelt.
- Die Dokumente bei jeder Änderung in 10 Sprachen. Kurz und einfach.

## Français

**L'objectif** : installer RPoRea **une seule fois** sur le serveur. RPoRea est l'édition supérieure et remplit aussi le rôle de Reaxt (`easy-web.tokyo/RPoRea`). Ainsi, open-easy-web, RPoem, aruaru-llm, open-directx, open-cuda, open-cpu et les autres n'ont plus à être réinstallés pour chaque site. Chaque site tourne comme un « clone d'ombre » (asynchrone, multi-thread) : économe en mémoire, rapide et sûr. **Pas d'API REST, pas de Node.js.**

**Où nous en sommes (honnêtement)** : fait : le point d'entrée de RPoRea et le lien avec Python, la transmission conditionnelle des identifiants, la base de l'hébergement par clones d'ombre (vrais processus lancés et aiguillés dans open-web-server et RPoem), la limite de concurrence d'aruaru-llm (en production). Pas encore : l'installation et le déploiement en un seul endroit, l'isolation par site (utilisateurs séparés, limites de mémoire), l'appel d'open-cuda, open-directx et open-cpu comme services partagés, les pages publiques. L'objectif ci-dessus est une cible, pas quelque chose de déjà terminé.

- Usage : d'abord depuis Rust. Les autres langages passent par `rporea-ffi` (ABI C).
- Briques web : avec HTML5, CSS3 et TypeScript en Rust. Peu de HTML/CSS, tokio et WASM là où ils conviennent, pas de REST, TypeScript d'abord.
- Les projets sœurs (Reaxt, RPoRea, open-cosmo) avancent toujours ensemble.
- Documents en 10 langues à chaque mise à jour. Courts et simples.

## Español

**El objetivo**: instalar RPoRea **una sola vez** en el servidor. RPoRea es la edición superior y también hace el trabajo de Reaxt (`easy-web.tokyo/RPoRea`). Así, open-easy-web, RPoem, aruaru-llm, open-directx, open-cuda, open-cpu y los demás no hay que instalarlos de nuevo para cada sitio. Cada sitio funciona como un «clon de sombra» (asíncrono, multihilo): poca memoria, rápido y seguro. **Sin API REST y sin Node.js.**

**Dónde estamos (con franqueza)**: hecho: el punto de entrada de RPoRea y el enlace con Python, el reenvío condicional de credenciales, la base del alojamiento con clones de sombra (procesos reales iniciados y enrutados en open-web-server y RPoem), el límite de concurrencia de aruaru-llm (en producción). Falta: instalación y despliegue en un solo lugar, aislamiento por sitio (usuarios separados, límites de memoria), llamar a open-cuda, open-directx y open-cpu como servicios compartidos, las páginas públicas. El objetivo de arriba es una meta, no algo ya terminado.

- Uso: primero desde Rust. Los demás lenguajes usan `rporea-ffi` (ABI de C).
- Piezas web: junto con HTML5, CSS3 y TypeScript en Rust. Poco HTML/CSS, tokio y WASM donde encajen, sin REST, TypeScript primero.
- Los proyectos hermanos (Reaxt, RPoRea, open-cosmo) se desarrollan siempre juntos.
- Los documentos, en 10 idiomas en cada cambio. Cortos y sencillos.

## Português (Brasil)

**O objetivo**: instalar o RPoRea **uma única vez** no servidor. O RPoRea é a edição superior e também faz o trabalho do Reaxt (`easy-web.tokyo/RPoRea`). Assim, open-easy-web, RPoem, aruaru-llm, open-directx, open-cuda, open-cpu e os demais não precisam ser instalados de novo para cada site. Cada site roda como um "clone de sombra" (assíncrono, multithread): pouca memória, rápido e seguro. **Sem API REST e sem Node.js.**

**Onde estamos (com sinceridade)**: pronto: o ponto de entrada do RPoRea e a ligação com Python, o repasse condicional de credenciais, a base da hospedagem com clones de sombra (processos reais iniciados e roteados no open-web-server e no RPoem), o limite de concorrência do aruaru-llm (em produção). Ainda não: instalação e implantação em um só lugar, isolamento por site (usuários separados, limites de memória), chamar open-cuda, open-directx e open-cpu como serviços compartilhados, as páginas públicas. O objetivo acima é uma meta, não algo já concluído.

- Uso: primeiro pelo Rust. As outras linguagens usam o `rporea-ffi` (ABI C).
- Peças web: junto com HTML5, CSS3 e TypeScript em Rust. Pouco HTML/CSS, tokio e WASM onde couberem, sem REST, TypeScript primeiro.
- Os projetos irmãos (Reaxt, RPoRea, open-cosmo) são sempre feitos juntos.
- Documentos em 10 idiomas a cada mudança. Curtos e simples.

## Русский

**Конечная цель**: установить RPoRea на сервер **только один раз**. RPoRea — старшая версия и заодно выполняет роль Reaxt (`easy-web.tokyo/RPoRea`). Тогда open-easy-web, RPoem, aruaru-llm, open-directx, open-cuda, open-cpu и остальное не нужно ставить заново для каждого сайта. Каждый сайт работает как «теневой клон» (асинхронно, в нескольких потоках): мало памяти, быстро и безопасно. **Без REST API и без Node.js.**

**Где мы сейчас (честно)**: готово: точка входа RPoRea и связь с Python, условная передача учётных данных, основа хостинга на «теневых клонах» (реальные процессы запускаются и распределяются в open-web-server и RPoem), ограничение параллельности в aruaru-llm (работает в продакшене). Ещё нет: установка и развёртывание в одном месте, изоляция по сайтам (отдельные пользователи, лимиты памяти), вызов open-cuda, open-directx и open-cpu как общих служб, публичные страницы. Цель выше — это задача на будущее, а не уже готовое.

- Использование: сначала из Rust. Другие языки — через `rporea-ffi` (C ABI).
- Веб-части: вместе с HTML5, CSS3 и TypeScript на Rust. Мало HTML/CSS, tokio и WASM — где уместно, без REST, сначала TypeScript.
- Сёстры (Reaxt, RPoRea, open-cosmo) всегда развиваются вместе.
- Документы при каждом изменении — на 10 языках. Коротко и просто.


## 2026-11-19 追記(3姉妹の同時更新): 複数サブグラフの問い合わせ分割と `@key` 結合 / Query splitting and `@key` joins

**日本語(やさしい説明)**: open-cosmo のゲートウェイが、複数のサブグラフにまたがる問い合わせを扱えるようになりました。(1) 読み取り専用の問い合わせで、ルートのフィールドが別々のサブグラフにあるときは、サブグラフごとに分けて同時に実行し、結果を1つにまとめます。(2) `@key` つきの型(エンティティ)は、サブグラフAが返した「id」を使って、別のサブグラフBの `_entities` に問い合わせ、足りないフィールドを足します(1段のみ)。変更系(mutation)をまたぐ問い合わせ、リモートとローカルの混在、フラグメント/ディレクティブつきは、従来どおり拒否します。試験はゲートウェイ44件・federation13件が成功(擬似サブグラフでの確認。Reaxt実機を複数並べた通しは未了)。Reaxt のサブグラフ(`_service{sdl}`・`__schema`)はそのまま対応しており、Reaxt側の変更は不要でした。
**English (simple)**: The open-cosmo gateway now handles queries that span several subgraphs. (1) A read-only query whose root fields live in different subgraphs is split, run at the same time, and merged. (2) For a type with `@key` (an entity), the gateway takes the id returned by subgraph A and asks subgraph B through `_entities` for the missing fields (one level only). Mutations across subgraphs, a mix of remote and local fields, and fragments/directives are still rejected. Tests pass (gateway 44, federation 13) against fake subgraphs; a run with several real Reaxt servers is not done yet. Reaxt's subgraph (`_service{sdl}`, `__schema`) already works, so Reaxt needed no change.
**未了 / Open**: 複数段の入れ子の結合・`@requires`/`@provides`・実Reaxtを複数並べた通し・ミューテーション後の再取得。Nested (multi-level) joins, `@requires`/`@provides`, a real multi-Reaxt run.

## 2026-11-19 追記: 実サーバーでの振り分けを確認(3姉妹共通) / Routing verified on real servers (shared by the three sisters)

### 日本語
`Reaxt` に2つ目のデモサーバー `examples/counter/src/bin/serve_greeter.rs`(GraphQL のフィールド名 `demo_v1_Greeter_Visit`)を追加し、カウンターのサーバー(`demo_v1_Counter_Inc`)と2台並べて、`open-cosmo` のゲートウェイ経由で実機確認しました。結果は次のとおりです。

- カウンターの変更はカウンターのサーバーへ、グリーターの変更はグリーターのサーバーへ、それぞれ正しく届き、数値は別々に増えました(カウンター 1→2、グリーター 1)。
- 2台にまたがる変更系(mutation)は、仕様どおり拒否されました。
- 注意: Reaxt のサブグラフは、引数を `input: $変数` の形でしか受け付けず、値は `null`(入力なし)が必要です。リテラルの引数は未対応です。
- 読み取り専用の問い合わせを2台にまたがって分割する経路は、Reaxt のサンプルが読み取りフィールドを `reaxt_ready`(共有)しか持たないため、実機では未確認です。擬似サブグラフでの試験は成功しています。

### English
We added a second Reaxt demo server, `examples/counter/src/bin/serve_greeter.rs` (GraphQL field `demo_v1_Greeter_Visit`), ran it next to the counter server (`demo_v1_Counter_Inc`), and checked both through the `open-cosmo` gateway on a real machine.

- A counter mutation reached the counter server and a greeter mutation reached the greeter server, and each counter advanced on its own (counter 1 then 2, greeter 1).
- A mutation spanning both servers was rejected, as designed.
- Note: a Reaxt subgraph accepts arguments only in the form `input: $variable`, and the value must be `null` (no input). Literal arguments are not supported yet.
- The read-only split across two servers was not checked on real servers, because the Reaxt examples expose only one read field, `reaxt_ready`, which both share. Tests against fake subgraphs pass.

**未了 / Open**: Reaxt のサブグラフの引数リテラル対応、読み取りフィールドを持つ実サンプルでの分割の通し確認、複数段の結合、`@requires` / `@provides`。 Literal arguments in the Reaxt subgraph, a real split run with an example that has read fields, multi-level joins, `@requires` / `@provides`.

## 2026-11-19 追記(続き): Reaxt のサブグラフが引数リテラルに対応(3姉妹共通) / Literal arguments in the Reaxt subgraph (shared by the three sisters)

### 日本語
`Reaxt` のサブグラフ(`reaxt-actions/src/subgraph.rs`)が、`input: $変数` に加えて **`input:` のリテラル**を受け付けるようになりました。オブジェクト・配列・文字列・数値・真偽値・`null`・列挙値、そして入れ子の中の `$変数` が使えます(例: `input: {by: 1, tags: ["x"], n: $n}`)。列挙値は文字列として渡します。壊れたリテラルはそのフィールドだけ `invalid_argument` のエラーになり、他のフィールドには影響しません。ブロック文字列(`"""`)と `input` 以外の引数名は未対応です。`Reaxt` の試験(`reaxt-actions` 10件)に成功しました。前の追記にある「リテラルは未対応」は、この更新で解消しています。

### English
The Reaxt subgraph (`reaxt-actions/src/subgraph.rs`) now accepts **literal values for `input:`** as well as `input: $variable`. Objects, lists, strings, numbers, booleans, `null`, enum values, and `$variable` references nested inside them all work (for example `input: {by: 1, tags: ["x"], n: $n}`). Enum values are passed as strings. A malformed literal fails only its own field with `invalid_argument` and leaves the other fields alone. Block strings (`"""`) and argument names other than `input` are not supported. The Reaxt tests pass (`reaxt-actions`, 10 tests). The "literals are not supported" remark in the previous note is resolved by this update.

**未了 / Open**: 読み取りフィールドを持つ実サンプルでの分割の通し確認、複数段の結合、`@requires` / `@provides`。 A real split run with an example that has read fields, multi-level joins, `@requires` / `@provides`.

## 2026-11-19 追記(続き2): `@key` 結合が入れ子に対応(3姉妹共通) / `@key` joins now work at any depth (shared by the three sisters)

### 日本語
`open-cosmo` のゲートウェイの `@key` 結合が、1段だけでなく**入れ子の段にも**対応しました。たとえば `{ me { name reviews manager { name reviews } } }` のように、`me` の中の `manager`(同じ型 `User`)でも、別のサブグラフ(reviews)にしかない `reviews` を取れます。浅い段から順に結合し、結合のために足した `id` と `__typename` は、最後に結果から取り除きます。試験はゲートウェイ全体で45件に成功しました(擬似サブグラフ)。**未了**: `@requires` / `@provides`、`@key` が複数のフィールドにまたがる入れ子の鍵、変更系の後の再取得、読み取りフィールドを持つ実際の Reaxt サンプルでの通し確認。

### English
The `@key` join in the `open-cosmo` gateway now works at **any depth of nesting**, not only one level. For example, in `{ me { name reviews manager { name reviews } } }` the `manager` inside `me` (also a `User`) can get `reviews`, which only another subgraph (reviews) defines. Joins run from the shallowest level down, and the `id` and `__typename` that were added only for the join are removed from the final result. Tests pass across the gateway (45 tests, fake subgraphs). **Open**: `@requires` / `@provides`, nested composite keys, re-fetching after a mutation, and a real run with a Reaxt example that has read fields.

## 2026-11-19 追記: 4姉妹プロジェクト(`open-cosmo`・`RPoem`・`Reaxt`・`RPoRea`)を同時に開発 / The four sister projects are developed together

### 日本語
これまでの「3姉妹」(`Reaxt`・`RPoRea`・`open-cosmo`)に、土台である **`RPoem`** を加えた **4姉妹**を、常に同時に開発します(ユーザー指示)。役割は次のとおりです。

- **`RPoem`**: 土台(アプリケーションサーバー、Tomcat 相当。管理 API は Connect / gRPC)。
- **`Reaxt`**: Rust で書く画面とサーバーアクション。Cosmo 向けのサブグラフ入口 `POST /_reaxt/graphql` を持つ。
- **`RPoRea`**: `RPoem` と `Reaxt` を合わせた土台。Rust 以外からは C ABI(`rporea-ffi`)で使う。
- **`open-cosmo`**: 複数のサブグラフをまとめるゲートウェイ(WunderGraph Cosmo 相当)。

**決まり**: どれかの接点(Cosmo のサブグラフ形式・manifest・Connect・`x-reaxt`・管理 API)を変えたら、4つの README・CLAUDE・PORTING・POLICY を同じ日に、日本語とネイティブに近い英語で更新する。最新の到達点: ゲートウェイの複数サブグラフ分割、`@key` 結合(入れ子の段にも対応)、Reaxt の `input:` リテラル対応(いずれも push 済み)。未了: `@requires` / `@provides`、変更系の後の再取得、読み取りフィールドを持つ実際の Reaxt サンプルでの通し確認。

### English
The former "three sisters" (`Reaxt`, `RPoRea`, `open-cosmo`) now include their foundation, **`RPoem`**, making **four sister projects** that are always developed together (user instruction). Their roles:

- **`RPoem`**: the foundation (an application server, comparable to Tomcat; its admin API uses Connect / gRPC).
- **`Reaxt`**: UI and server actions written in Rust, with a Cosmo-compatible subgraph entry, `POST /_reaxt/graphql`.
- **`RPoRea`**: the base that joins `RPoem` and `Reaxt`; callers other than Rust use its C ABI (`rporea-ffi`).
- **`open-cosmo`**: the gateway that combines several subgraphs (comparable to WunderGraph Cosmo).

**Rule**: when an interface between them changes (the Cosmo subgraph format, the manifest, Connect, `x-reaxt`, the admin API), update the README, CLAUDE, PORTING and POLICY of all four on the same day, in Japanese and natural English. Latest state: the gateway splits queries across subgraphs and supports `@key` joins (including nested levels), and Reaxt accepts `input:` literals (all pushed). Open: `@requires` / `@provides`, re-fetching after a mutation, and a real run with a Reaxt example that has read fields.

## 2026-11-19 追記(続き3): 実サーバー2台での「読み取りの分割」を確認、Reaxt に読み取り用アクションを追加(4姉妹共通) / Read-query split verified on two real servers; read actions added to Reaxt (shared by the four sisters)

### 日本語
- **Reaxt の規約**: メソッド名が `Get` / `List` / `Read` で始まるアクション(例: `GetCount`、`GetVisits`)は、Cosmo 向けのサブグラフで GraphQL の **Query** として公開されます。それ以外は従来どおり **Mutation** です。変更用のアクションが1つも無いときは、SDL に `Mutation` 型を出しません。`reaxt_ready` は引き続き全サブグラフで共有されます。
- **実機での確認**: カウンター(`GetCount`)とグリーター(`GetVisits`)の2台を `open-cosmo` のゲートウェイ越しに動かし、`query { c: demo_v1_Counter_GetCount g: demo_v1_Greeter_GetVisits }` が、2台に分けて実行され、`{"data":{"c":2,"g":1}}` と1つの結果にまとまることを確認しました。2台にまたがる変更系は、従来どおり「読み取り専用の問い合わせだけが分割できる」というメッセージで拒否されます。`input:` のリテラルも実機で通りました。
- **試験**: Reaxt の `reaxt-actions` は11件に成功。
- **未了**: `@requires` / `@provides`、変更系の後の再取得。

### English
- **Reaxt convention**: actions whose method name starts with `Get`, `List` or `Read` (for example `GetCount`, `GetVisits`) are exposed by the Cosmo subgraph as GraphQL **Query** fields; all other actions remain **Mutation** fields. When there are no mutating actions, the SDL has no `Mutation` type. `reaxt_ready` is still shared by every subgraph.
- **Verified on a real machine**: with the counter (`GetCount`) and greeter (`GetVisits`) servers running behind the `open-cosmo` gateway, `query { c: demo_v1_Counter_GetCount g: demo_v1_Greeter_GetVisits }` was split across the two servers and merged into one answer, `{"data":{"c":2,"g":1}}`. A mutation spanning both is still rejected, now with the message that only read-only queries can be split. `input:` literals also worked on the real servers.
- **Tests**: `reaxt-actions` passes 11 tests.
- **Open**: `@requires` / `@provides`, and re-fetching after a mutation.

## 2026-11-19 追記(続き4): `@external`・`@provides`・`@requires` に対応(4姉妹共通) / `@external`, `@provides` and `@requires` supported (shared by the four sisters)

### 日本語
`open-cosmo` のゲートウェイの `@key` 結合が、Apollo Federation の残りの主なディレクティブにも対応しました。

- **`@external`**: サブグラフAが `@external` と宣言したフィールドは、Aでは解決できないものとして扱い、実際に持つサブグラフBへの結合(`_entities`)の対象にします。
- **`@provides`**: 親のフィールドが `@provides(fields: "...")` で供給すると宣言したフィールドだけは、`@external` でもAに任せ、結合しません(余計な往復を減らします)。
- **`@requires`**: サブグラフBのフィールドが `@requires(fields: "a b")` で他のフィールドを必要とするとき、(1) そのフィールドが選ばれたときだけ、`a` と `b` をサブグラフAの問い合わせに足して先に取得し、(2) エンティティ表現(representation)に鍵と一緒に入れて B の `_entities` へ送ります。クライアントが選んでいない `a` と `b` は、最後に結果から取り除きます。Aが必須フィールドを供給できないときは、結合せずに従来どおり転送します。
- **設計の根拠(調査)**: Apollo の [Subgraph 仕様](https://www.apollographql.com/docs/graphos/reference/federation/subgraph-spec/)(`_entities` は表現と同じ順で返す、`@requires` のフィールドは表現に含める)と、[apollographql/router](https://github.com/apollographql/router) の計画器(必須フィールドは、要求したフィールドが選ばれたときだけ取得する修正、`FetchNode` の `requires`)を参考にしました。コードは流用せず、Rust で一から実装しています。
- **試験**: ゲートウェイ47件・federation 15件に成功(擬似サブグラフ)。
- **未了**: 必須フィールドが「さらに別のサブグラフ」にある場合(3者以上の連鎖)、`@skip` / `@include` つきの条件、変更系の後の再取得。

### English
The `@key` join in the `open-cosmo` gateway now also handles the other main Apollo Federation directives.

- **`@external`**: a field that subgraph A marks `@external` is treated as not resolvable by A, and is joined (through `_entities`) from the subgraph B that really owns it.
- **`@provides`**: a field that the parent declares as supplied by `@provides(fields: "...")` stays with A even though it is `@external`, so no extra round trip is made.
- **`@requires`**: when a field of subgraph B uses `@requires(fields: "a b")`, the gateway (1) adds `a` and `b` to the query sent to subgraph A, only when that field is selected, and (2) sends them in the entity representation together with the key to B's `_entities`. Fields the client did not select are removed from the final result. If A cannot supply a required field, the query is forwarded as before without a join.
- **Design sources (research)**: the Apollo [Subgraph specification](https://www.apollographql.com/docs/graphos/reference/federation/subgraph-spec/) (`_entities` returns entities in the order of the representations; `@requires` fields belong in the representation) and the planner in [apollographql/router](https://github.com/apollographql/router) (required fields are fetched only when the requiring field is selected; `FetchNode` has `requires`). No code was copied; this is a fresh Rust implementation.
- **Tests**: gateway 47 and federation 15 pass (fake subgraphs).
- **Open**: required fields that live in yet another subgraph (chains of three or more), `@skip` / `@include` conditions, and re-fetching after a mutation.

## 2026-11-19 追記(続き5): 変更系(mutation)が返したエンティティの結合に対応(4姉妹共通) / Joins on entities returned by a mutation (shared by the four sisters)

### 日本語
`open-cosmo` のゲートウェイが、**変更系(mutation)が返したエンティティ**にも `@key` 結合を行うようになりました。たとえば `mutation { createUser { name reviews } }` で、`createUser` は users サブグラフが実行し、`reviews`(別のサブグラフにしかないフィールド)は、返ってきた `id` を使って reviews サブグラフの `_entities`(読み取り専用)から足します。

- **変更は1回だけ**: mutation は元のサブグラフで1回だけ実行し、結合のための追加の呼び出しは読み取り専用の `_entities` だけです(二重実行しません)。試験で、users 側が `mutation` を1回だけ、reviews 側が `_entities` だけを受けたことを確認しました。
- **従来どおり拒否**: 変更系が複数のサブグラフにまたがる問い合わせは、拒否します(原子的にできないため)。
- **試験**: ゲートウェイ48件に成功(擬似サブグラフ)。
- **未了**: 必須フィールドが別のサブグラフにある3者以上の連鎖、`@skip` / `@include` つきの条件。

### English
The `open-cosmo` gateway now also performs `@key` joins on **entities returned by a mutation**. For example, in `mutation { createUser { name reviews } }` the users subgraph runs `createUser`, and `reviews` (a field only another subgraph defines) is added from the reviews subgraph's read-only `_entities`, using the returned `id`.

- **The mutation runs once**: it is executed once at its own subgraph, and the only extra calls are read-only `_entities` calls (nothing runs twice). The test confirms that the users side saw exactly one `mutation` and the reviews side saw only `_entities`.
- **Still rejected**: mutations that span several subgraphs (they cannot be made atomic).
- **Tests**: the gateway passes 48 tests (fake subgraphs).
- **Open**: chains of three or more subgraphs where a required field lives in yet another subgraph, and `@skip` / `@include` conditions.

## 2026-11-19 追記(続き6): `@skip` / `@include` と、3者以上の連鎖に対応(4姉妹共通) / `@skip` / `@include` and chains of three or more subgraphs (shared by the four sisters)

### 日本語
`open-cosmo` のゲートウェイの結合が、次の2つに対応しました。

- **`@skip(if:)` / `@include(if:)`**: 条件は、リテラルか、問い合わせの時点で分かっている変数の値で、ゲートウェイが**計画時に**判定します。除外されたフィールドは問い合わせから外し、そのフィールドのためだけに必要だった別のサブグラフへの呼び出しも行いません。これ以外のディレクティブや、値の分からない変数は、従来どおり結合せずに転送します。
- **3者以上の連鎖**: `@requires` の必須フィールドが、最初のサブグラフでも結合先でもない**さらに別のサブグラフ**にあるとき、その別のサブグラフへの結合を先に行い、得た値を結合先の表現(representation)に入れます(例: catalog → pricing で `price` を取得 → shipping の `shippingEstimate` に渡す)。結合には順位を付けて、必要な順に実行します(最大4段)。クライアントが選んでいない必須フィールドは、最後に結果から取り除きます。どのサブグラフも供給できないときは、結合せずに転送します。
- **試験**: ゲートウェイ51件に成功(擬似サブグラフ)。
- **未了**: 実際のサーバー(Reaxt 以外のサブグラフ)での `@key` / `@requires` の通し確認、`@skip` / `@include` をフラグメント内で使う場合(フラグメントは分割・結合の対象外のまま)。

### English
The join in the `open-cosmo` gateway now handles two more cases.

- **`@skip(if:)` / `@include(if:)`**: the condition is a literal or a variable whose value is already known, and the gateway decides it **at planning time**. An excluded field is dropped from the query, and a call to another subgraph that was needed only for that field is not made. Other directives, and variables with no known value, are forwarded as before, without a join.
- **Chains of three or more subgraphs**: when a `@requires` field lives in **yet another subgraph** (neither the first one nor the one being joined), the gateway first joins that third subgraph and puts the value into the representation sent onward (for example: catalog, then pricing supplies `price`, then shipping computes `shippingEstimate`). Joins get a rank and run in dependency order (up to four levels). Required fields the client did not select are removed from the final result. If no subgraph can supply a field, the query is forwarded without a join.
- **Tests**: the gateway passes 51 tests (fake subgraphs).
- **Open**: a run of `@key` / `@requires` against real servers other than Reaxt, and `@skip` / `@include` inside fragments (fragments are still outside the split and join planner).

## 2026-11-19 追記(続き7): 本物のフェデレーション実装2台で `@key` 結合を確認し、ルート型の名前の不具合を修正(4姉妹共通) / `@key` join verified against two real federation servers; root-type-name bug fixed (shared by the four sisters)

### 日本語
擬似サブグラフだけでなく、**独立した本物の GraphQL サーバー**でも確認しました。`async-graphql` のフェデレーション機能(`_service { sdl }` と `_entities` を自前で実装)で users と reviews の2台を立て、`open-cosmo` のゲートウェイ越しに `{ me { name reviews } }` を送ると、`{"name":"user-7","reviews":["review-of-7"]}` と正しく結合されました(試験はゲートウェイ52件に成功)。

- **見つかった不具合と修正**: 実際のサーバーは、ルート型の名前が `Query` とは限りません(今回は `QueryA`)。ゲートウェイは `Query` / `Mutation` 固定で探していたため、結合が働かず、元の問い合わせがそのまま転送されて失敗しました。内省(introspection)でルート型の名前を取得し、内部では `Query` / `Mutation` に正規化するようにして直しました。擬似サブグラフだけの試験では見つからなかった不具合です。
- **未了**: `@requires` と `@provides` を本物のサーバーで確認する試験(今回は `@key` のみ)、Reaxt 自身のエンティティ(`@key`)対応、フラグメント内のディレクティブ。

### English
We verified the join not only with fake subgraphs but also with **independent, real GraphQL servers**. Two servers (users and reviews) were built with `async-graphql`'s own federation support (its own `_service { sdl }` and `_entities`); sending `{ me { name reviews } }` through the `open-cosmo` gateway returned the correctly joined `{"name":"user-7","reviews":["review-of-7"]}` (the gateway passes 52 tests).

- **Bug found and fixed**: a real server's root type is not always called `Query` (here it was `QueryA`). The gateway looked for `Query` / `Mutation` by name, so no join happened and the original query was forwarded unchanged, which failed. The gateway now reads the root type names through introspection and normalizes them to `Query` / `Mutation` internally. The fake-subgraph tests alone could not have found this.
- **Open**: tests of `@requires` and `@provides` against real servers (this run covered `@key` only), entity (`@key`) support in Reaxt itself, and directives inside fragments.

## 2026-11-19 追記(続き8): 本物の SDL での `@external` / `@requires` / `@provides` の読み取りを確認、Reaxt のエンティティ対応は見送り(4姉妹共通) / Directives read from real SDL; Reaxt entities deliberately not added (shared by the four sisters)

### 日本語
- **確認**: `async-graphql` が実際に生成する SDL(`#[graphql(external)]`・`#[graphql(requires = "price")]`・`#[graphql(provides = "price")]` つき)を、ゲートウェイのモデルが正しく読むことを試験で確認しました(`@key`・`@external`・`@requires`・`@provides` の4種。ゲートウェイ53件に成功)。手書きの SDL だけでなく、実際の実装が出力する書式でも読めます。ただし、`@requires` と `@provides` を使った**結合の実行**を本物のサーバー2台で通すところまでは、まだ行っていません(実行の確認は擬似サブグラフでの試験のみ)。
- **判断(見送り)**: Reaxt 自身を `@key` のエンティティにすることは**しません**。Reaxt のアクションは `JSON` スカラーを返し、GraphQL の型(フィールドの構造)を持たないためです。型を持たせるには Reaxt のアクション定義に型情報を足す大きな設計変更が必要で、現状の用途(アクションの呼び出しと読み取り)には不要です。必要になったら、`Wire` 型から GraphQL の型を生成する形を検討します。
- **未了**: `@requires` / `@provides` を使った結合を、本物のサーバー2台で実行する試験、フラグメント内のディレクティブ。

### English
- **Verified**: the gateway model correctly reads the SDL that `async-graphql` really generates (with `#[graphql(external)]`, `#[graphql(requires = "price")]` and `#[graphql(provides = "price")]`): `@key`, `@external`, `@requires` and `@provides` (the gateway passes 53 tests). It reads the format produced by a real implementation, not only hand-written SDL. However, running a **join that uses `@requires` and `@provides`** against two real servers has not been done yet (that execution is covered only by fake-subgraph tests).
- **Decision (not added)**: Reaxt itself will **not** become a `@key` entity. Reaxt actions return the `JSON` scalar and have no GraphQL type (no field structure). Giving them one would need a large design change in the action definitions, and the current use (calling actions and reading values) does not need it. If it is ever needed, we will consider generating GraphQL types from the `Wire` types.
- **Open**: running joins that use `@requires` / `@provides` against two real servers, and directives inside fragments.

## 2026-11-19 続き9: Reaxtのエンティティ対応(訂正)
**日本語**: 直前の「Reaxtのエンティティ対応は見送り」は**撤回**します。アクションに型を持たせました(`Wire::gql_type`・`wire_struct!`がGraphQLの型を出し、`ModuleActions::entity`で`@key`のエンティティとして公開)。サブグラフのSDLは型付き(`Visitor @key(fields: "id")`)になり、`_entities`と`... on 型 { }`・別名・`__typename`の選択に対応。実プロセス(`serve_greeter`)でSDLと`_entities`の応答を確認済み、`reaxt-actions`の試験12件成功。未了: ゲートウェイ越しの2サーバー結合の実機確認、フラグメント内の`@skip/@include`。
**English**: The earlier "Reaxt entities postponed" note is **withdrawn**. Actions now carry GraphQL types (`Wire::gql_type`, `wire_struct!`), and `ModuleActions::entity` exposes an `@key` entity. The subgraph SDL is typed (`Visitor @key(fields: "id")`) and `_entities`, `... on Type { }`, aliases and `__typename` work. Verified on a real process (`serve_greeter`): SDL and `_entities`; 12 `reaxt-actions` tests pass. Open: a real two-server join through the gateway, and `@skip/@include` inside fragments.

**2026-11-19 続き10(実機)**: ゲートウェイ(`OPEN_RUNO_SUBGRAPHS="counter=...,greeter=..."`、`name=url`形式)の後ろに実Reaxtサーバー2つ(`serve`・`serve_greeter`、どちらも型付き`Visitor`エンティティ)を置き、`{ GetVisitor { __typename id visits count } }` が `visits`(greeter)と`count`(counter)を`@key`で結合して返すことを確認(visits=2・count=1、別名・`__typename`も可)。2サーバーにまたがる変更系は仕様どおり拒否。 / **Real run**: through the gateway with two real typed Reaxt servers, one query returned `visits` (greeter) and `count` (counter) joined by `@key` (visits=2, count=1; alias and `__typename` work); a mutation spanning both servers is rejected by design.

**2026-11-19 続き11**: ゲートウェイの分割経路で、型条件なしのインラインフラグメント(`... @include(if: $x) { }`)を親へ展開し、`@skip/@include`をその場で評価するようにした(試験追加、`remote_subgraphs`の24件成功)。型条件つき(`... on T`)・名前付きフラグメントは、インターフェース/ユニオンで誤るため従来どおり通常転送。結合計画(`@key`)側の経路への展開は未了。 / **Fragments**: in the split path, untyped inline fragments are flattened into the parent and `@skip/@include` are evaluated (24 tests pass). Typed (`... on T`) and named fragments still use plain forwarding because flattening is wrong for interfaces/unions. The join-planner path is not covered yet.


## 2026-11-19 いまの状況と、これから作る機能(4姉妹: open-cosmo・RPoem・Reaxt・RPoRea)

### 日本語(やさしい説明)
**4つは、いつも一緒に作ります。** `RPoem`は土台、`open-cosmo`は入口(玄関)、`Reaxt`は画面とサーバーの仕事を書く道具、`RPoRea`はRPoemとReaxtをくっつけたものです。1つを変えたら、ほかの3つへの影響も確かめ、4つの README・CLAUDE・PORTING・POLICY を同じ日に直します。

**もうできていること**
- `Reaxt`のサーバーが、Cosmo(複数のサーバーをまとめる仕組み)に参加できる(`POST /_reaxt/graphql`)。
- 入口(ゲートウェイ)が、1つの質問を複数のサーバーに分けて聞き、答えをまとめる。
- `@key`で、別々のサーバーが持つ同じ人・物の情報をつなぐ(入れ子や、4段までの連鎖もOK)。
- `@requires`と`@provides`(必要な項目を先に取る・持っている項目をそのまま使う)。
- `@skip`と`@include`(項目を出す・出さないの切り替え)。型条件のないフラグメントの中でも動く。
- `Reaxt`のアクションが型(項目の形)を持てる。`@key`のエンティティとして公開できる。
- 本物のサーバー2つを入口につないで、1つの質問で結合できることを確認した。

**これから作る機能(完成予定)**
1. 結合の計画(`@key`)でも、フラグメントを使えるようにする。
2. `... on 型`(型条件つきのフラグメント)に、安全な範囲で対応する。
3. 変更(書き込み)のあとに、最新の値を取り直す。
4. `Reaxt`の`Option`(あるかもしれない値)や、もっと複雑な型への対応。
5. `open-cosmo`のルーターへ、Reaxtのサーバーをもっと広く実際につなぐ試験。
6. `RPoRea`をRust以外(Pythonなど)から使う入口(C ABI)の、Mojoでの確認、Mac/Android/iPhoneへの広がり。

**進め方のきまり**: 終わっていない仕事は、**確認を取らず、自動で次々に進めます**。作ったら試験をして、本物のサーバーで確かめてから「できた」と書きます。秘密(パスワードなど)は書きません。新しいRESTは作りません。

### English (simple words, easy to read)
**We always build these four together.** `RPoem` is the base. `open-cosmo` is the front door. `Reaxt` is the tool for writing screens and server jobs. `RPoRea` joins RPoem and Reaxt. If we change one, we check the other three too, and we update the README, CLAUDE, PORTING and POLICY of all four on the same day.

**What already works**
- A `Reaxt` server can join Cosmo (a way to combine many servers) at `POST /_reaxt/graphql`.
- The front door (gateway) splits one question across many servers and puts the answers together.
- `@key` joins the same person or thing that lives in different servers (nested joins and chains up to four steps work).
- `@requires` and `@provides` (fetch needed fields first; reuse fields a server already has).
- `@skip` and `@include` (show or hide a field). They also work inside fragments that have no type condition.
- `Reaxt` actions can have types (the shape of the fields) and can be shown as `@key` entities.
- We ran two real servers behind the gateway and joined them with one question.

**What we plan to build (the finish line)**
1. Let the join planner (`@key`) use fragments too.
2. Support `... on Type` (fragments with a type condition) in a safe way.
3. Re-read the newest values after a write.
4. Support `Option` (a value that may be missing) and richer types in `Reaxt`.
5. Wider real tests that connect Reaxt servers to the `open-cosmo` router.
6. Use `RPoRea` from other languages (the C ABI): check it with Mojo, then Mac, Android and iPhone.

**How we work**: we keep going on unfinished jobs **without asking for confirmation**. We test, check on real servers, and only then say "done". We never write secrets (such as passwords). We do not add new REST APIs.
