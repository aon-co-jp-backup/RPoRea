# RPoRea お引越し・再開

兄弟ディレクトリに `Reaxt` `RPoem` `open-cosmo` `open-web-server`(RPoem が依存)を clone してから始める。

## 日本語

**現状(正直に)**: できている=RPoRea の入口と Python 連携、認証ヘッダの条件付き転送、分身ホスティングの土台(open-web-server と RPoem で、実プロセスの起動と振り分けを確認)、aruaru-llm の同時実行制限(本番で稼働)。まだ=1か所への入れ替え配備、サイトごとの分離(別ユーザー・メモリ制限)、open-cuda・open-directx・open-cpu を共有基盤として呼ぶ統合、公開ページ。上の完成イメージは目標で、全部できたわけではありません。

1. ビルド: `cargo build -p rporea-ffi`
2. 試験: `py -3 bindings/python/test_rporea.py`(`python binding OK` が出れば成功)
3. 次の作業: open-cosmo のルーターに Reaxt のサブグラフを合成して確認する。

## English

**Where we are (honestly)**: done: the RPoRea entry point and Python support, conditional forwarding of credentials, the base of shadow-clone hosting (real processes started and routed in open-web-server and RPoem), aruaru-llm's concurrency limit (live in production). Not yet: one-place install and deployment, per-site isolation (separate users, memory limits), calling open-cuda, open-directx and open-cpu as shared services, the public pages. The goal above is a target, not something already finished.

1. Build: `cargo build -p rporea-ffi`
2. Test: `py -3 bindings/python/test_rporea.py` (success prints `python binding OK`)
3. Next: join Reaxt's subgraph with the open-cosmo router and check it.

## 简体中文

**现状(如实说明)**:已完成:RPoRea 的入口和 Python 支持、认证头的有条件转发、分身托管的基础(在 open-web-server 和 RPoem 中验证了真实进程的启动和分流)、aruaru-llm 的并发限制(已在线上运行)。尚未完成:一处安装与部署、按网站隔离(独立用户、内存限制)、把 open-cuda、open-directx、open-cpu 作为共享服务来调用、公开页面。上面的样子是目标,并非全部已经做好。

1. 构建:`cargo build -p rporea-ffi`
2. 测试:`py -3 bindings/python/test_rporea.py`(成功时会显示 `python binding OK`)
3. 下一步:把 Reaxt 的子图接到 open-cosmo 路由器上并验证。

## 繁體中文(台灣)

**現況(如實說明)**:已完成:RPoRea 的入口與 Python 支援、認證標頭的有條件轉送、分身代管的基礎(在 open-web-server 與 RPoem 中驗證了真實行程的啟動與分流)、aruaru-llm 的並行上限(已在正式環境運作)。尚未完成:一處安裝與部署、依網站隔離(獨立使用者、記憶體限制)、把 open-cuda、open-directx、open-cpu 當作共用服務來呼叫、公開頁面。上面的樣子是目標,並非全都已經做好。

1. 建置:`cargo build -p rporea-ffi`
2. 測試:`py -3 bindings/python/test_rporea.py`(成功時會顯示 `python binding OK`)
3. 下一步:把 Reaxt 的子圖接到 open-cosmo 路由器上並驗證。

## 한국어

**현재 상태(솔직하게)**: 된 것: RPoRea 진입점과 Python 연동, 인증 헤더의 조건부 전달, 분신 호스팅의 기초(open-web-server와 RPoem에서 실제 프로세스 시작과 분배를 확인), aruaru-llm의 동시 실행 제한(운영 중). 아직인 것: 한 곳 설치·배포, 사이트별 분리(별도 사용자, 메모리 제한), open-cuda·open-directx·open-cpu를 공유 서비스로 호출하는 통합, 공개 페이지. 위의 모습은 목표이며 전부 끝난 것은 아닙니다.

1. 빌드: `cargo build -p rporea-ffi`
2. 시험: `py -3 bindings/python/test_rporea.py` (성공하면 `python binding OK`가 나옵니다)
3. 다음 작업: Reaxt의 서브그래프를 open-cosmo 라우터에 붙여서 확인합니다.

## Deutsch

**Wo wir stehen (ehrlich)**: Fertig: der Einstieg von RPoRea und die Python-Anbindung, die bedingte Weitergabe von Anmeldedaten, die Grundlage für Schattendoppelgänger-Hosting (echte Prozesse in open-web-server und RPoem gestartet und verteilt), das Gleichzeitigkeitslimit von aruaru-llm (produktiv). Noch offen: Installation und Auslieferung an einer Stelle, Trennung pro Website (eigene Benutzer, Speicherlimits), Aufruf von open-cuda, open-directx und open-cpu als gemeinsame Dienste, die öffentlichen Seiten. Das Ziel oben ist ein Vorhaben, nicht etwas schon Fertiges.

1. Bauen: `cargo build -p rporea-ffi`
2. Testen: `py -3 bindings/python/test_rporea.py` (bei Erfolg erscheint `python binding OK`)
3. Als Nächstes: den Subgraph von Reaxt mit dem open-cosmo-Router zusammenbringen und prüfen.

## Français

**Où nous en sommes (honnêtement)** : fait : le point d'entrée de RPoRea et le lien avec Python, la transmission conditionnelle des identifiants, la base de l'hébergement par clones d'ombre (vrais processus lancés et aiguillés dans open-web-server et RPoem), la limite de concurrence d'aruaru-llm (en production). Pas encore : l'installation et le déploiement en un seul endroit, l'isolation par site (utilisateurs séparés, limites de mémoire), l'appel d'open-cuda, open-directx et open-cpu comme services partagés, les pages publiques. L'objectif ci-dessus est une cible, pas quelque chose de déjà terminé.

1. Compiler : `cargo build -p rporea-ffi`
2. Tester : `py -3 bindings/python/test_rporea.py` (en cas de succès : `python binding OK`)
3. Ensuite : relier le sous-graphe de Reaxt au routeur open-cosmo et le vérifier.

## Español

**Dónde estamos (con franqueza)**: hecho: el punto de entrada de RPoRea y el enlace con Python, el reenvío condicional de credenciales, la base del alojamiento con clones de sombra (procesos reales iniciados y enrutados en open-web-server y RPoem), el límite de concurrencia de aruaru-llm (en producción). Falta: instalación y despliegue en un solo lugar, aislamiento por sitio (usuarios separados, límites de memoria), llamar a open-cuda, open-directx y open-cpu como servicios compartidos, las páginas públicas. El objetivo de arriba es una meta, no algo ya terminado.

1. Compilar: `cargo build -p rporea-ffi`
2. Probar: `py -3 bindings/python/test_rporea.py` (si sale bien, muestra `python binding OK`)
3. Siguiente: unir el subgrafo de Reaxt con el enrutador de open-cosmo y comprobarlo.

## Português (Brasil)

**Onde estamos (com sinceridade)**: pronto: o ponto de entrada do RPoRea e a ligação com Python, o repasse condicional de credenciais, a base da hospedagem com clones de sombra (processos reais iniciados e roteados no open-web-server e no RPoem), o limite de concorrência do aruaru-llm (em produção). Ainda não: instalação e implantação em um só lugar, isolamento por site (usuários separados, limites de memória), chamar open-cuda, open-directx e open-cpu como serviços compartilhados, as páginas públicas. O objetivo acima é uma meta, não algo já concluído.

1. Compilar: `cargo build -p rporea-ffi`
2. Testar: `py -3 bindings/python/test_rporea.py` (se der certo, aparece `python binding OK`)
3. Próximo passo: juntar o subgrafo do Reaxt ao roteador do open-cosmo e conferir.

## Русский

**Где мы сейчас (честно)**: готово: точка входа RPoRea и связь с Python, условная передача учётных данных, основа хостинга на «теневых клонах» (реальные процессы запускаются и распределяются в open-web-server и RPoem), ограничение параллельности в aruaru-llm (работает в продакшене). Ещё нет: установка и развёртывание в одном месте, изоляция по сайтам (отдельные пользователи, лимиты памяти), вызов open-cuda, open-directx и open-cpu как общих служб, публичные страницы. Цель выше — это задача на будущее, а не уже готовое.

1. Сборка: `cargo build -p rporea-ffi`
2. Проверка: `py -3 bindings/python/test_rporea.py` (при успехе выводится `python binding OK`)
3. Дальше: подключить подграф Reaxt к маршрутизатору open-cosmo и проверить.

## 2026-11-19 更新: 複数サブグラフにまたがる問い合わせ(3姉妹共通) / Cross-subgraph queries (shared by the three sisters)

### 日本語
`open-cosmo` のゲートウェイが、複数のサブグラフ(別々のサーバー)にまたがる問い合わせを扱えるようになりました。`Reaxt`・`RPoRea`・`open-cosmo` は常に同時に開発しているため、3つのリポジトリに同じ内容を書いています。

- **分割と統合**: 読み取り専用の問い合わせで、最上位のフィールドが別々のサブグラフにあるときは、サブグラフごとの問い合わせに分け、同時に実行して、結果を1つにまとめます。
- **`@key` による結合(1段)**: `@key` を持つ型について、サブグラフAが返した `id` を使い、サブグラフBの `_entities` に問い合わせて、Aにないフィールドを足します。
- **従来どおり拒否するもの**: 変更系(mutation)が複数のサブグラフにまたがる問い合わせ、リモートとローカルが混ざる問い合わせ、フラグメントやディレクティブを含む分割対象の問い合わせ。
- **確認済み**: 擬似サブグラフでの試験(gateway 44件、federation 13件)に成功。Reaxt の実サーバーが返す SDL も、`compose` と管理 RPC `ComposeFederation` で受け付けられます。
- **未了**: 複数段にわたる入れ子の結合、`@requires` / `@provides`、実際の Reaxt サーバーを複数並べた通し確認(今のサンプルは同じフィールド名を持つため、振り分けの確認にはサンプルの追加が必要)。

### English
The `open-cosmo` gateway can now handle queries that span several subgraphs (separate servers). `Reaxt`, `RPoRea` and `open-cosmo` are always developed together, so all three repositories carry this same note.

- **Split and merge**: for a read-only query whose top-level fields live in different subgraphs, the gateway builds one sub-query per subgraph, runs them concurrently, and merges the results.
- **`@key` joins (one level)**: for a type that has `@key`, the gateway takes the `id` returned by subgraph A and asks subgraph B, through `_entities`, for the fields A does not have.
- **Still rejected**: mutations that span subgraphs, queries that mix remote and local fields, and split queries that contain fragments or directives.
- **Verified**: tests against fake subgraphs pass (gateway 44, federation 13). The SDL served by a real Reaxt server is accepted by `compose` and by the admin RPC `ComposeFederation`.
- **Open**: joins nested more than one level, `@requires` / `@provides`, and a run with several real Reaxt servers (the current example exposes the same field names, so a second example is needed to check routing).

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
