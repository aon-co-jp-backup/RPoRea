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
