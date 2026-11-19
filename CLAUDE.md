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

## 2026-11-19 方針と再開点 / Policy and resume points (10 languages)

### 日本語
- **使い方**: RPoRea/RPoem は基本 **Rust から**使う。Python・Mojo・RMojo・C など Rust 以外は C ABI(`rporea-ffi`、`include/rporea.h`)経由。Python は Windows と Linux(WSL の Ubuntu)で実機確認済み、Mojo/RMojo は未検証。
- **対応 OS**: 最初は Windows と Linux。リリースが完成してから Mac・Android・iPhone・iPad・スーパーコンピューター(未着手)。
- **Web の部品**: 開発中の Rust 版 HTML5+CSS3+TypeScript(`rhtml5`・`rcss3`・`RS-TypeScript`、`RS-JavaScript` は雛形)と一緒に Web サイト・Web アプリ・アプリを作る想定。HTML5/CSS3 は極力少なく、tokio と WASM は適材適所、REST は使わない、**TypeScript の Rust 版を優先**。詳細は `docs/WEB-STACK.md`。
- **文書のルール**: README/CLAUDE/PORTING は更新のたびに 10 か国語(日・英・簡中・繁中〈台湾〉・韓・独・仏・西・葡〈ブラジル〉・露)に、ネイティブに近い自然さで、IT・Web に詳しい人が一般の人にも分かりやすく親切に書く。
- **未了**: Mojo/RMojo の検証、Mac・モバイル・スーパーコンピューター、open-cosmo ルーターへの Reaxt サブグラフの実合成、`.ts` → JS → アセット登録の CLI。

### English
- **Usage**: RPoRea/RPoem are used **from Rust** first. Python, Mojo, RMojo, C and other languages go through the C ABI (`rporea-ffi`, `include/rporea.h`). Python is verified on Windows and Linux (WSL Ubuntu); Mojo/RMojo are not verified.
- **Platforms**: Windows and Linux first. Mac, Android, iPhone, iPad and supercomputers after the release is complete (not started).
- **Web parts**: meant to be used with our Rust HTML5 + CSS3 + TypeScript (`rhtml5`, `rcss3`, `RS-TypeScript`; `RS-JavaScript` is a scaffold) for websites, web apps and apps. As little HTML5/CSS3 as possible, tokio and WASM where each fits, no REST, **Rust TypeScript first**. See `docs/WEB-STACK.md`.
- **Docs rule**: with every update, README/CLAUDE/PORTING are translated into 10 languages (ja, en, zh-CN, zh-TW〈Taiwan〉, ko, de, fr, es, pt〈Brazil〉, ru), natural and close to native, written the way a knowledgeable IT/Web person would explain it kindly to general readers.
- **Open**: verify Mojo/RMojo; Mac, mobile, supercomputers; compose Reaxt's subgraph through the open-cosmo router; a CLI for `.ts` → JS → asset registration.

### 简体中文
- **用法**:RPoRea/RPoem 首先**直接用 Rust**。Python、Mojo、RMojo、C 等其他语言通过 C ABI(`rporea-ffi`、`include/rporea.h`)调用。Python 已在 Windows 和 Linux(WSL 的 Ubuntu)上验证,Mojo/RMojo 尚未验证。
- **支持的系统**:先做 Windows 和 Linux;正式版完成后再支持 Mac、Android、iPhone、iPad 和超级计算机(尚未开始)。
- **Web 组件**:设想与开发中的 Rust 版 HTML5+CSS3+TypeScript(`rhtml5`、`rcss3`、`RS-TypeScript`;`RS-JavaScript` 只有骨架)一起开发网站、Web 应用和各类应用。尽量少用 HTML5/CSS3,tokio 与 WASM 因地制宜,不用 REST,**优先做 TypeScript 的 Rust 版**。详见 `docs/WEB-STACK.md`。
- **文档规则**:每次更新 README/CLAUDE/PORTING 时,都翻译成 10 种语言(日、英、简中、繁中〈台湾〉、韩、德、法、西、葡〈巴西〉、俄),力求接近母语的自然表达,以熟悉 IT 和 Web 的人向普通读者耐心讲解的口吻来写。
- **待办**:验证 Mojo/RMojo;Mac、移动端、超级计算机;通过 open-cosmo 路由器实际组合 Reaxt 子图;`.ts` → JS → 资源注册的命令行工具。

### 繁體中文(台灣)
- **用法**:RPoRea/RPoem 首先**直接使用 Rust**。Python、Mojo、RMojo、C 等其他語言則透過 C ABI(`rporea-ffi`、`include/rporea.h`)呼叫。Python 已在 Windows 與 Linux(WSL 的 Ubuntu)上驗證,Mojo/RMojo 尚未驗證。
- **支援的作業系統**:先做 Windows 與 Linux;正式版完成後再支援 Mac、Android、iPhone、iPad 與超級電腦(尚未開始)。
- **Web 元件**:構想是與開發中的 Rust 版 HTML5+CSS3+TypeScript(`rhtml5`、`rcss3`、`RS-TypeScript`;`RS-JavaScript` 僅有骨架)一起開發網站、Web 應用程式與各種應用程式。盡量少用 HTML5/CSS3,tokio 與 WASM 因地制宜,不使用 REST,**優先開發 TypeScript 的 Rust 版**。詳見 `docs/WEB-STACK.md`。
- **文件規則**:每次更新 README/CLAUDE/PORTING 時,都翻譯成 10 種語言(日、英、簡中、繁中〈台灣〉、韓、德、法、西、葡〈巴西〉、俄),力求貼近母語的自然表達,以熟悉 IT 與 Web 的人向一般讀者親切說明的口吻來寫。
- **待辦**:驗證 Mojo/RMojo;Mac、行動裝置、超級電腦;透過 open-cosmo 路由器實際組合 Reaxt 子圖;`.ts` → JS → 資源註冊的命令列工具。

### 한국어
- **사용법**: RPoRea/RPoem은 우선 **Rust에서 직접** 씁니다. Python, Mojo, RMojo, C 같은 다른 언어는 C ABI(`rporea-ffi`, `include/rporea.h`)로 호출합니다. Python은 Windows와 Linux(WSL의 Ubuntu)에서 확인했고, Mojo/RMojo는 아직 검증하지 못했습니다.
- **지원 OS**: 먼저 Windows와 Linux. 정식 릴리스가 완성된 뒤 Mac, Android, iPhone, iPad, 슈퍼컴퓨터(아직 시작 전).
- **웹 부품**: 개발 중인 Rust판 HTML5+CSS3+TypeScript(`rhtml5`, `rcss3`, `RS-TypeScript`; `RS-JavaScript`는 뼈대뿐)와 함께 웹사이트, 웹 앱, 일반 앱을 만드는 것이 목표입니다. HTML5/CSS3는 가능한 한 적게, tokio와 WASM은 알맞은 곳에, REST는 쓰지 않고, **TypeScript의 Rust판을 우선**합니다. 자세한 내용은 `docs/WEB-STACK.md`.
- **문서 규칙**: README/CLAUDE/PORTING은 갱신할 때마다 10개 언어(일, 영, 중국어 간체, 중국어 번체〈대만〉, 한, 독, 불, 서, 포르투갈어〈브라질〉, 러)로 번역하되, 원어민에 가까운 자연스러움으로, IT·웹에 밝은 사람이 일반 독자에게 친절하게 설명하는 어조로 씁니다.
- **남은 일**: Mojo/RMojo 검증, Mac·모바일·슈퍼컴퓨터, open-cosmo 라우터로 Reaxt 서브그래프 실제 결합, `.ts` → JS → 에셋 등록 CLI.

### Deutsch
- **Nutzung**: RPoRea/RPoem werden zuerst **direkt aus Rust** genutzt. Python, Mojo, RMojo, C und andere Sprachen gehen über die C-ABI (`rporea-ffi`, `include/rporea.h`). Python ist unter Windows und Linux (Ubuntu in WSL) geprüft, Mojo/RMojo noch nicht.
- **Betriebssysteme**: zuerst Windows und Linux; nach Fertigstellung der Veröffentlichung Mac, Android, iPhone, iPad und Supercomputer (nicht begonnen).
- **Web-Bausteine**: gedacht für den Einsatz mit unserem Rust-HTML5 + CSS3 + TypeScript (`rhtml5`, `rcss3`, `RS-TypeScript`; `RS-JavaScript` ist nur ein Gerüst) für Websites, Web-Apps und Apps. So wenig HTML5/CSS3 wie möglich, tokio und WASM dort, wo sie passen, kein REST, **TypeScript in Rust hat Vorrang**. Siehe `docs/WEB-STACK.md`.
- **Dokumentationsregel**: Bei jeder Änderung werden README/CLAUDE/PORTING in 10 Sprachen übersetzt (ja, en, zh-CN, zh-TW〈Taiwan〉, ko, de, fr, es, pt〈Brasilien〉, ru) – natürlich und nah an Muttersprache, so geschrieben, wie ein IT-/Web-Kundiger es Laien freundlich erklären würde.
- **Offen**: Mojo/RMojo prüfen; Mac, Mobilgeräte, Supercomputer; Reaxt-Subgraph über den open-cosmo-Router tatsächlich zusammenführen; ein CLI für `.ts` → JS → Asset-Registrierung.

### Français
- **Utilisation** : RPoRea/RPoem s'utilisent d'abord **directement depuis Rust**. Python, Mojo, RMojo, C et les autres langages passent par l'ABI C (`rporea-ffi`, `include/rporea.h`). Python est vérifié sous Windows et Linux (Ubuntu sous WSL) ; Mojo/RMojo ne le sont pas encore.
- **Systèmes** : d'abord Windows et Linux ; Mac, Android, iPhone, iPad et supercalculateurs une fois la version finalisée (rien n'est commencé).
- **Briques web** : prévu pour fonctionner avec notre HTML5 + CSS3 + TypeScript en Rust (`rhtml5`, `rcss3`, `RS-TypeScript` ; `RS-JavaScript` n'est qu'une ébauche) pour des sites, des applications web et des applications. Le moins possible de HTML5/CSS3, tokio et WASM là où chacun convient, pas de REST, **le TypeScript en Rust d'abord**. Voir `docs/WEB-STACK.md`.
- **Règle de documentation** : à chaque mise à jour, README/CLAUDE/PORTING sont traduits en 10 langues (ja, en, zh-CN, zh-TW〈Taïwan〉, ko, de, fr, es, pt〈Brésil〉, ru), de façon naturelle et proche d'un natif, rédigés comme le ferait une personne qui connaît l'informatique et le web pour l'expliquer aimablement au grand public.
- **À faire** : vérifier Mojo/RMojo ; Mac, mobiles, supercalculateurs ; assembler réellement le sous-graphe de Reaxt via le routeur open-cosmo ; une CLI pour `.ts` → JS → enregistrement des ressources.

### Español
- **Uso**: RPoRea/RPoem se usan primero **directamente desde Rust**. Python, Mojo, RMojo, C y otros lenguajes pasan por la ABI de C (`rporea-ffi`, `include/rporea.h`). Python está verificado en Windows y Linux (Ubuntu en WSL); Mojo/RMojo todavía no.
- **Sistemas**: primero Windows y Linux; Mac, Android, iPhone, iPad y supercomputadoras cuando la versión esté terminada (sin empezar).
- **Piezas web**: pensado para usarse con nuestro HTML5 + CSS3 + TypeScript en Rust (`rhtml5`, `rcss3`, `RS-TypeScript`; `RS-JavaScript` es solo un esqueleto) en sitios web, aplicaciones web y aplicaciones. Lo mínimo posible de HTML5/CSS3, tokio y WASM donde cada uno encaje, sin REST, **primero el TypeScript en Rust**. Véase `docs/WEB-STACK.md`.
- **Regla de documentación**: en cada actualización, README/CLAUDE/PORTING se traducen a 10 idiomas (ja, en, zh-CN, zh-TW〈Taiwán〉, ko, de, fr, es, pt〈Brasil〉, ru), con naturalidad cercana a la de un nativo, escritos como lo explicaría con amabilidad a un público general alguien que conoce bien la informática y la web.
- **Pendiente**: verificar Mojo/RMojo; Mac, móviles, supercomputadoras; componer de verdad el subgrafo de Reaxt con el enrutador de open-cosmo; una CLI para `.ts` → JS → registro de recursos.

### Português (Brasil)
- **Uso**: o RPoRea/RPoem são usados primeiro **direto do Rust**. Python, Mojo, RMojo, C e outras linguagens passam pela ABI C (`rporea-ffi`, `include/rporea.h`). O Python está verificado no Windows e no Linux (Ubuntu no WSL); Mojo/RMojo ainda não.
- **Sistemas**: primeiro Windows e Linux; Mac, Android, iPhone, iPad e supercomputadores depois que a versão estiver pronta (nada iniciado).
- **Peças web**: pensado para uso com o nosso HTML5 + CSS3 + TypeScript em Rust (`rhtml5`, `rcss3`, `RS-TypeScript`; `RS-JavaScript` é só um esqueleto) em sites, aplicativos web e aplicativos. O mínimo possível de HTML5/CSS3, tokio e WASM onde cada um faz sentido, sem REST, **TypeScript em Rust primeiro**. Veja `docs/WEB-STACK.md`.
- **Regra de documentação**: a cada atualização, README/CLAUDE/PORTING são traduzidos para 10 idiomas (ja, en, zh-CN, zh-TW〈Taiwan〉, ko, de, fr, es, pt〈Brasil〉, ru), de forma natural e próxima da de um nativo, escritos como uma pessoa que entende de TI e web explicaria com gentileza ao público em geral.
- **Pendente**: verificar Mojo/RMojo; Mac, celulares, supercomputadores; compor de fato o subgrafo do Reaxt pelo roteador do open-cosmo; uma CLI para `.ts` → JS → registro de recursos.

### Русский
- **Использование**: RPoRea/RPoem прежде всего используются **напрямую из Rust**. Python, Mojo, RMojo, C и другие языки работают через C ABI (`rporea-ffi`, `include/rporea.h`). Python проверен в Windows и Linux (Ubuntu в WSL); Mojo/RMojo пока нет.
- **Системы**: сначала Windows и Linux; Mac, Android, iPhone, iPad и суперкомпьютеры — после готовности релиза (не начато).
- **Веб-компоненты**: предполагается использование вместе с нашими HTML5 + CSS3 + TypeScript на Rust (`rhtml5`, `rcss3`, `RS-TypeScript`; `RS-JavaScript` — пока заготовка) для сайтов, веб-приложений и приложений. Как можно меньше HTML5/CSS3, tokio и WASM — там, где каждый уместен, без REST, **TypeScript на Rust — в приоритете**. См. `docs/WEB-STACK.md`.
- **Правило документации**: при каждом обновлении README/CLAUDE/PORTING переводятся на 10 языков (ja, en, zh-CN, zh-TW〈Тайвань〉, ko, de, fr, es, pt〈Бразилия〉, ru) естественно, близко к речи носителя, в духе того, как знаток IT и веба доброжелательно объяснил бы всё обычному читателю.
- **Осталось**: проверить Mojo/RMojo; Mac, мобильные устройства, суперкомпьютеры; реально собрать подграф Reaxt через маршрутизатор open-cosmo; CLI для цепочки `.ts` → JS → регистрация ресурсов.

