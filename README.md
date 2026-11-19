# RPoRea

**言語 / Languages**: [日本語](#日本語) · [English](#english) · [简体中文](#简体中文) · [繁體中文(台灣)](#繁體中文台灣) · [한국어](#한국어) · [Deutsch](#deutsch) · [Français](#français) · [Español](#español) · [Português (Brasil)](#português-brasil) · [Русский](#русский)

> 英語・中国語・韓国語・ドイツ語・フランス語・スペイン語・ポルトガル語・ロシア語は AI による翻訳で、ネイティブの方による確認はまだです(気づいた点は Issue でお知らせください)。

---

## 日本語

**RPoem と Reaxt を一つにしたものです。** RPoem は Rust 製のサーバー部品、Reaxt は画面もサーバーも Rust だけで書ける枠組みです。Reaxt で作ったアプリを、1 行で RPoem の上に載せて起動できます。

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

### どの言語から使えるか
基本は **Rust から**使います。Rust 以外の言語からは、共有ライブラリ(C ABI、`rporea-ffi`)経由で同じ機能を呼べます。

| 言語 | 状態 |
|---|---|
| Rust | 動作・試験あり |
| Python | **動作確認済み**(Windows と Linux〈WSL の Ubuntu〉)。`bindings/python/rporea.py`(標準ライブラリの ctypes だけ) |
| Mojo / RMojo | **未検証**(Mojo がこの PC に無いため)。同じ共有ライブラリを呼ぶ形で用意予定 |
| C / C++ | ヘッダ `include/rporea.h` あり |

```python
import rporea
app = rporea.App("My site")
app.asset("/", "text/html; charset=utf-8", "<h1>Hello</h1>")
app.method("demo.v1.Echo", "Say", lambda body: '{"ok": true}')   # POST /demo.v1.Echo/Say(REST ではなく Connect)
app.serve(8080)
```
詳しくは [`docs/BINDINGS.md`](docs/BINDINGS.md)。

### 対応 OS
今は **Windows と Linux**。リリースが完成してから **Mac・Android・iPhone・iPad・スーパーコンピューター**にも広げます(いずれも未着手)。

### 開発中の Rust 版 HTML5+CSS3+TypeScript と一緒に使う
RPoRea と RPoem は、開発中の **Rust 版の HTML5・CSS3・TypeScript**(`rhtml5`・`rcss3`・`RS-TypeScript`、JavaScript 実行エンジンの `RS-JavaScript` は雛形)と組み合わせて、Web サイト・Web アプリ・アプリを作る想定です。方針は次の 4 つ。
1. HTML5 と CSS3 の使用量は**できるだけ少なく**する。
2. tokio(サーバーの非同期)と WASM(ブラウザ内)は**適材適所**で使う。
3. **REST は使わない**(ブラウザからは Connect、サービス間は gRPC)。
4. JavaScript の Rust 版も進めるが、**TypeScript の Rust 版を優先**する。

各部品の現状と、つなぎ方の想定は [`docs/WEB-STACK.md`](docs/WEB-STACK.md)。

### 3姉妹リポジトリ(常に同時開発)
[`Reaxt`](https://github.com/aon-co-jp/Reaxt) / **RPoRea** / [`open-cosmo`](https://github.com/aon-co-jp/open-cosmo)(WunderGraph Cosmo 相当の Rust 製オープンソース版)。約束と接点の現状は [`docs/THREE-SISTERS.md`](docs/THREE-SISTERS.md)。3 つと親の [`RPoem`](https://github.com/aon-co-jp/RPoem) は兄弟ディレクトリに置きます(パス依存)。

### 現状(正直に)
- 動くもの: 入口(`start`/`serve`)、C ABI(`rporea-ffi`)と Python 結合(Windows・Linux で実機確認)、試験(Rust 側 3 件+ Python 1 本)。
- 未着手・未検証: open-cosmo のルーターとの合成確認、Mojo/RMojo、Mac・モバイル・スーパーコンピューター、`.ts` → JS → アセット登録を自動化する CLI、配備(SET)。
- ライセンス: Apache-2.0 OR MIT

---

## English

**RPoRea fuses RPoem and Reaxt.** RPoem is a Rust server toolkit; Reaxt lets you write both the UI and the server in Rust only. With one call, a Reaxt app runs on top of RPoem: `rporea::serve(app, addr)` (tracing and compression included; use `rporea::start` if you need the bound address and the task handle).

**Languages.** RPoRea is meant to be used **from Rust**. Other languages call the same features through a shared library with a C ABI (`rporea-ffi`, header `include/rporea.h`):
Rust — working and tested · Python — **verified on Windows and Linux (WSL Ubuntu)**, `bindings/python/rporea.py` uses only the standard `ctypes` · Mojo / RMojo — **not verified yet** (no Mojo on the dev machine) · C/C++ — header provided. See [`docs/BINDINGS.md`](docs/BINDINGS.md).

**Platforms.** Windows and Linux now. Mac, Android, iPhone, iPad and supercomputers once the release is complete (none started).

**Use it with the Rust HTML5 + CSS3 + TypeScript we are building.** RPoRea and RPoem are meant for websites, web apps and apps together with our Rust versions of HTML5, CSS3 and TypeScript (`rhtml5`, `rcss3`, `RS-TypeScript`; the JavaScript engine `RS-JavaScript` is only a scaffold). Principles: (1) use as little HTML5/CSS3 as possible; (2) use tokio (server-side async) and WASM (in the browser) where each fits; (3) no REST — Connect from browsers, gRPC between services; (4) a Rust JavaScript is in progress too, but the **Rust TypeScript comes first**. Details: [`docs/WEB-STACK.md`](docs/WEB-STACK.md).

**Three sister repositories, always developed together**: [`Reaxt`](https://github.com/aon-co-jp/Reaxt), **RPoRea**, [`open-cosmo`](https://github.com/aon-co-jp/open-cosmo) (an open-source Rust counterpart of WunderGraph Cosmo). Rules and the state of the touch points: [`docs/THREE-SISTERS.md`](docs/THREE-SISTERS.md). Keep them and the parent [`RPoem`](https://github.com/aon-co-jp/RPoem) as sibling directories (path dependencies).

**Status, honestly.** Working: the entry points, the C ABI and the Python binding (checked on real Windows and on Linux), tests. Not started / not verified: composing with the open-cosmo router, Mojo/RMojo, Mac/mobile/supercomputers, a CLI that automates `.ts` → JS → assets, deployment. License: Apache-2.0 OR MIT.

---

## 简体中文

**RPoRea 是 RPoem 与 Reaxt 的融合。** RPoem 是用 Rust 编写的服务器组件,Reaxt 则让界面和服务器端都只用 Rust 来写。只需一行代码,就能把 Reaxt 应用放到 RPoem 上运行:`rporea::serve(app, addr)`(已包含日志追踪和压缩;如果需要拿到实际监听的地址和任务句柄,请用 `rporea::start`)。

**可以用哪些语言?** RPoRea 的基本用法是**直接用 Rust**。其他语言通过带 C ABI 的共享库(`rporea-ffi`,头文件 `include/rporea.h`)调用同样的功能:Rust —— 可用且有测试;Python —— **已在 Windows 和 Linux(WSL 的 Ubuntu)上验证**,`bindings/python/rporea.py` 只用标准库的 `ctypes`;Mojo / RMojo —— **尚未验证**(开发机上没有 Mojo);C/C++ —— 提供头文件。详见 [`docs/BINDINGS.md`](docs/BINDINGS.md)。

**支持的系统。** 目前是 Windows 和 Linux。正式版完成后,再扩展到 Mac、Android、iPhone、iPad 和超级计算机(都还没开始)。

**与我们正在开发的 Rust 版 HTML5 + CSS3 + TypeScript 搭配使用。** RPoRea 和 RPoem 的设想是:与我们开发中的 Rust 版 HTML5、CSS3、TypeScript(`rhtml5`、`rcss3`、`RS-TypeScript`;JavaScript 引擎 `RS-JavaScript` 目前只有骨架)一起,用来开发网站、Web 应用和各类应用。原则有四条:(1)尽量少用 HTML5 和 CSS3;(2)tokio(服务器端异步)与 WASM(浏览器内)各取所长、因地制宜;(3)不用 REST——浏览器端用 Connect,服务之间用 gRPC;(4)JavaScript 的 Rust 版也在推进,但**优先做 TypeScript 的 Rust 版**。详见 [`docs/WEB-STACK.md`](docs/WEB-STACK.md)。

**三姐妹仓库(始终同步开发)**:[`Reaxt`](https://github.com/aon-co-jp/Reaxt)、**RPoRea**、[`open-cosmo`](https://github.com/aon-co-jp/open-cosmo)(相当于 WunderGraph Cosmo 的 Rust 开源版)。约定与接口现状见 [`docs/THREE-SISTERS.md`](docs/THREE-SISTERS.md)。请把它们和上层的 [`RPoem`](https://github.com/aon-co-jp/RPoem) 放在同级目录(路径依赖)。

**如实说明现状。** 已可用:入口函数、C ABI 和 Python 绑定(在真实的 Windows 与 Linux 上验证过)、测试。尚未开始或未验证:与 open-cosmo 路由器的组合验证、Mojo/RMojo、Mac/移动端/超级计算机、自动化 `.ts` → JS → 资源注册的命令行工具、部署。许可证:Apache-2.0 OR MIT。

---

## 繁體中文(台灣)

**RPoRea 是 RPoem 與 Reaxt 的整合。** RPoem 是以 Rust 撰寫的伺服器元件,Reaxt 則讓畫面與伺服器端都只用 Rust 來寫。只要一行程式碼,就能把 Reaxt 應用程式放到 RPoem 上執行:`rporea::serve(app, addr)`(已內建追蹤記錄與壓縮;若需要取得實際監聽的位址與工作控制代碼,請改用 `rporea::start`)。

**可以用哪些語言?** RPoRea 的基本用法是**直接使用 Rust**。其他語言則透過具備 C ABI 的共用函式庫(`rporea-ffi`,標頭檔 `include/rporea.h`)呼叫相同的功能:Rust —— 可運作且有測試;Python —— **已在 Windows 與 Linux(WSL 的 Ubuntu)上驗證**,`bindings/python/rporea.py` 只使用標準函式庫的 `ctypes`;Mojo / RMojo —— **尚未驗證**(開發用電腦上沒有 Mojo);C/C++ —— 提供標頭檔。詳見 [`docs/BINDINGS.md`](docs/BINDINGS.md)。

**支援的作業系統。** 目前是 Windows 與 Linux。正式版完成後,再擴大到 Mac、Android、iPhone、iPad 與超級電腦(皆尚未開始)。

**搭配我們正在開發的 Rust 版 HTML5 + CSS3 + TypeScript 使用。** RPoRea 與 RPoem 的構想是:和我們開發中的 Rust 版 HTML5、CSS3、TypeScript(`rhtml5`、`rcss3`、`RS-TypeScript`;JavaScript 引擎 `RS-JavaScript` 目前僅有骨架)一起,用來開發網站、Web 應用程式與各種應用程式。原則有四項:(1)盡量少用 HTML5 與 CSS3;(2)tokio(伺服器端非同步)與 WASM(瀏覽器內)各取所長、因地制宜;(3)不使用 REST——瀏覽器端用 Connect,服務之間用 gRPC;(4)JavaScript 的 Rust 版也在進行,但**優先開發 TypeScript 的 Rust 版**。詳見 [`docs/WEB-STACK.md`](docs/WEB-STACK.md)。

**三姊妹倉庫(始終同步開發)**:[`Reaxt`](https://github.com/aon-co-jp/Reaxt)、**RPoRea**、[`open-cosmo`](https://github.com/aon-co-jp/open-cosmo)(相當於 WunderGraph Cosmo 的 Rust 開源版)。約定與介接點的現況請見 [`docs/THREE-SISTERS.md`](docs/THREE-SISTERS.md)。請將它們與上層的 [`RPoem`](https://github.com/aon-co-jp/RPoem) 放在同一層目錄(路徑相依)。

**如實說明現況。** 已可運作:入口函式、C ABI 與 Python 綁定(已在真實的 Windows 與 Linux 上驗證)、測試。尚未開始或未驗證:與 open-cosmo 路由器的整合驗證、Mojo/RMojo、Mac/行動裝置/超級電腦、自動化 `.ts` → JS → 資源註冊的命令列工具、部署。授權:Apache-2.0 OR MIT。

---

## 한국어

**RPoRea는 RPoem과 Reaxt를 하나로 합친 프로젝트입니다.** RPoem은 Rust로 만든 서버 부품이고, Reaxt는 화면과 서버를 모두 Rust만으로 작성할 수 있게 해 주는 프레임워크입니다. 코드 한 줄이면 Reaxt 앱을 RPoem 위에서 실행할 수 있습니다: `rporea::serve(app, addr)`(추적 로그와 압축이 기본 포함되어 있으며, 실제로 바인드된 주소와 작업 핸들이 필요하면 `rporea::start`를 쓰세요).

**어떤 언어에서 쓸 수 있나요?** 기본은 **Rust에서 직접** 쓰는 것입니다. 그 밖의 언어는 C ABI 공유 라이브러리(`rporea-ffi`, 헤더 `include/rporea.h`)를 통해 같은 기능을 호출합니다: Rust — 동작하며 테스트 있음 · Python — **Windows와 Linux(WSL의 Ubuntu)에서 확인 완료**, `bindings/python/rporea.py`는 표준 라이브러리 `ctypes`만 사용 · Mojo / RMojo — **아직 검증하지 못했습니다**(개발 PC에 Mojo가 없음) · C/C++ — 헤더 제공. 자세한 내용은 [`docs/BINDINGS.md`](docs/BINDINGS.md)를 보세요.

**지원 OS.** 지금은 Windows와 Linux입니다. 정식 릴리스가 완성된 뒤에 Mac, Android, iPhone, iPad, 슈퍼컴퓨터로 넓혀 갑니다(모두 아직 시작 전).

**개발 중인 Rust판 HTML5 + CSS3 + TypeScript와 함께 쓰기.** RPoRea와 RPoem은 개발 중인 Rust판 HTML5, CSS3, TypeScript(`rhtml5`, `rcss3`, `RS-TypeScript`; JavaScript 엔진 `RS-JavaScript`는 아직 뼈대뿐)와 함께 웹사이트, 웹 앱, 일반 앱을 만드는 것을 염두에 두고 있습니다. 방침은 네 가지입니다. (1) HTML5와 CSS3는 가능한 한 적게 사용합니다. (2) tokio(서버 쪽 비동기)와 WASM(브라우저 안)은 각각 알맞은 곳에 씁니다. (3) REST는 쓰지 않습니다 — 브라우저에서는 Connect, 서비스 사이에서는 gRPC. (4) JavaScript의 Rust판도 진행하지만 **TypeScript의 Rust판을 우선**합니다. 자세한 내용은 [`docs/WEB-STACK.md`](docs/WEB-STACK.md).

**세 자매 저장소(항상 함께 개발)**: [`Reaxt`](https://github.com/aon-co-jp/Reaxt), **RPoRea**, [`open-cosmo`](https://github.com/aon-co-jp/open-cosmo)(WunderGraph Cosmo에 해당하는 Rust 오픈소스판). 약속과 접점의 현재 상태는 [`docs/THREE-SISTERS.md`](docs/THREE-SISTERS.md)에 있습니다. 이 셋과 상위 [`RPoem`](https://github.com/aon-co-jp/RPoem)은 같은 폴더 아래 나란히 두세요(경로 의존).

**현재 상태(솔직하게).** 동작하는 것: 진입 함수, C ABI와 Python 바인딩(실제 Windows와 Linux에서 확인), 테스트. 아직 시작하지 않았거나 검증하지 못한 것: open-cosmo 라우터와의 결합 확인, Mojo/RMojo, Mac·모바일·슈퍼컴퓨터, `.ts` → JS → 에셋 등록을 자동화하는 CLI, 배포. 라이선스: Apache-2.0 OR MIT.

---

## Deutsch

**RPoRea verbindet RPoem und Reaxt.** RPoem ist ein Server-Baukasten in Rust, mit Reaxt schreibt man Oberfläche und Server ausschließlich in Rust. Mit einem einzigen Aufruf läuft eine Reaxt-App auf RPoem: `rporea::serve(app, addr)` (inklusive Tracing und Kompression; wer die tatsächlich gebundene Adresse und das Task-Handle braucht, nimmt `rporea::start`).

**Aus welchen Sprachen lässt es sich nutzen?** RPoRea ist dafür gedacht, **direkt aus Rust** verwendet zu werden. Andere Sprachen rufen dieselben Funktionen über eine Shared Library mit C-ABI auf (`rporea-ffi`, Header `include/rporea.h`): Rust – funktioniert, mit Tests · Python – **auf Windows und Linux (Ubuntu unter WSL) geprüft**, `bindings/python/rporea.py` nutzt nur das `ctypes` der Standardbibliothek · Mojo / RMojo – **noch nicht geprüft** (auf dem Entwicklungsrechner ist kein Mojo installiert) · C/C++ – Header vorhanden. Mehr dazu in [`docs/BINDINGS.md`](docs/BINDINGS.md).

**Betriebssysteme.** Derzeit Windows und Linux. Sobald die Veröffentlichung steht, folgen Mac, Android, iPhone, iPad und Supercomputer (noch nichts davon begonnen).

**Zusammen mit dem in Arbeit befindlichen Rust-HTML5 + CSS3 + TypeScript nutzen.** RPoRea und RPoem sind dafür gedacht, gemeinsam mit unseren Rust-Versionen von HTML5, CSS3 und TypeScript (`rhtml5`, `rcss3`, `RS-TypeScript`; die JavaScript-Engine `RS-JavaScript` ist bisher nur ein Gerüst) Websites, Web-Apps und Apps zu bauen. Vier Grundsätze: (1) HTML5 und CSS3 so wenig wie möglich einsetzen; (2) tokio (asynchron auf dem Server) und WASM (im Browser) dort verwenden, wo sie jeweils passen; (3) kein REST – Connect vom Browser, gRPC zwischen Diensten; (4) auch ein JavaScript in Rust ist in Arbeit, aber **das TypeScript in Rust hat Vorrang**. Einzelheiten: [`docs/WEB-STACK.md`](docs/WEB-STACK.md).

**Drei Schwester-Repositories, immer gemeinsam entwickelt**: [`Reaxt`](https://github.com/aon-co-jp/Reaxt), **RPoRea**, [`open-cosmo`](https://github.com/aon-co-jp/open-cosmo) (das quelloffene Rust-Gegenstück zu WunderGraph Cosmo). Regeln und aktueller Stand der Schnittstellen: [`docs/THREE-SISTERS.md`](docs/THREE-SISTERS.md). Die drei und das übergeordnete [`RPoem`](https://github.com/aon-co-jp/RPoem) liegen als Geschwisterverzeichnisse nebeneinander (Pfadabhängigkeiten).

**Stand, ehrlich gesagt.** Funktioniert: die Einstiegsfunktionen, die C-ABI und die Python-Anbindung (auf echtem Windows und Linux geprüft), Tests. Nicht begonnen oder ungeprüft: das Zusammenspiel mit dem open-cosmo-Router, Mojo/RMojo, Mac/Mobilgeräte/Supercomputer, ein CLI, das `.ts` → JS → Assets automatisiert, Deployment. Lizenz: Apache-2.0 OR MIT.

---

## Français

**RPoRea fusionne RPoem et Reaxt.** RPoem est une boîte à outils serveur en Rust ; Reaxt permet d'écrire l'interface comme le serveur uniquement en Rust. Un seul appel suffit pour faire tourner une application Reaxt sur RPoem : `rporea::serve(app, addr)` (traçage et compression inclus ; utilisez `rporea::start` si vous avez besoin de l'adresse réellement liée et du handle de la tâche).

**Depuis quels langages ?** RPoRea est conçu pour être utilisé **directement depuis Rust**. Les autres langages appellent les mêmes fonctions via une bibliothèque partagée à ABI C (`rporea-ffi`, en-tête `include/rporea.h`) : Rust — fonctionne, avec tests · Python — **vérifié sous Windows et Linux (Ubuntu sous WSL)**, `bindings/python/rporea.py` n'utilise que le module `ctypes` de la bibliothèque standard · Mojo / RMojo — **pas encore vérifié** (pas de Mojo sur la machine de développement) · C/C++ — en-tête fourni. Détails dans [`docs/BINDINGS.md`](docs/BINDINGS.md).

**Systèmes pris en charge.** Aujourd'hui Windows et Linux. Une fois la version finalisée, Mac, Android, iPhone, iPad et supercalculateurs suivront (rien n'est encore commencé).

**À utiliser avec notre HTML5 + CSS3 + TypeScript en Rust, en cours de développement.** RPoRea et RPoem sont pensés pour créer des sites web, des applications web et des applications avec nos versions Rust de HTML5, CSS3 et TypeScript (`rhtml5`, `rcss3`, `RS-TypeScript` ; le moteur JavaScript `RS-JavaScript` n'est pour l'instant qu'une ébauche). Quatre principes : (1) utiliser le moins possible de HTML5 et de CSS3 ; (2) employer tokio (asynchrone côté serveur) et WASM (dans le navigateur) là où chacun convient ; (3) pas de REST — Connect depuis le navigateur, gRPC entre les services ; (4) un JavaScript en Rust avance aussi, mais **le TypeScript en Rust est prioritaire**. Détails : [`docs/WEB-STACK.md`](docs/WEB-STACK.md).

**Trois dépôts sœurs, toujours développés ensemble** : [`Reaxt`](https://github.com/aon-co-jp/Reaxt), **RPoRea**, [`open-cosmo`](https://github.com/aon-co-jp/open-cosmo) (l'équivalent open source en Rust de WunderGraph Cosmo). Règles et état des points de contact : [`docs/THREE-SISTERS.md`](docs/THREE-SISTERS.md). Placez-les, avec le dépôt parent [`RPoem`](https://github.com/aon-co-jp/RPoem), dans des dossiers voisins (dépendances par chemin).

**État des lieux, en toute honnêteté.** Ça fonctionne : les points d'entrée, l'ABI C et la liaison Python (vérifiées sur un vrai Windows et sous Linux), les tests. Pas commencé ou non vérifié : l'assemblage avec le routeur open-cosmo, Mojo/RMojo, Mac/mobiles/supercalculateurs, une CLI qui automatise `.ts` → JS → ressources, le déploiement. Licence : Apache-2.0 OR MIT.

---

## Español

**RPoRea une RPoem y Reaxt.** RPoem es un conjunto de piezas de servidor escritas en Rust; con Reaxt se escribe tanto la interfaz como el servidor solo en Rust. Con una sola llamada, una aplicación Reaxt se ejecuta sobre RPoem: `rporea::serve(app, addr)` (incluye trazas y compresión; usa `rporea::start` si necesitas la dirección realmente enlazada y el manejador de la tarea).

**¿Desde qué lenguajes se puede usar?** RPoRea está pensado para usarse **directamente desde Rust**. Otros lenguajes llaman a las mismas funciones mediante una biblioteca compartida con ABI de C (`rporea-ffi`, cabecera `include/rporea.h`): Rust — funciona y tiene pruebas · Python — **verificado en Windows y Linux (Ubuntu en WSL)**, `bindings/python/rporea.py` solo usa `ctypes` de la biblioteca estándar · Mojo / RMojo — **aún sin verificar** (el equipo de desarrollo no tiene Mojo) · C/C++ — cabecera incluida. Más detalles en [`docs/BINDINGS.md`](docs/BINDINGS.md).

**Sistemas compatibles.** Ahora, Windows y Linux. Cuando la versión esté terminada, se ampliará a Mac, Android, iPhone, iPad y supercomputadoras (nada de eso ha empezado).

**Úsalo junto con nuestro HTML5 + CSS3 + TypeScript en Rust, que está en desarrollo.** RPoRea y RPoem están pensados para crear sitios web, aplicaciones web y aplicaciones junto con nuestras versiones en Rust de HTML5, CSS3 y TypeScript (`rhtml5`, `rcss3`, `RS-TypeScript`; el motor de JavaScript `RS-JavaScript` es por ahora solo un esqueleto). Cuatro principios: (1) usar el mínimo posible de HTML5 y CSS3; (2) emplear tokio (asincronía en el servidor) y WASM (dentro del navegador) allí donde cada uno encaje; (3) nada de REST: Connect desde el navegador y gRPC entre servicios; (4) también avanza un JavaScript en Rust, pero **el TypeScript en Rust tiene prioridad**. Detalles en [`docs/WEB-STACK.md`](docs/WEB-STACK.md).

**Tres repositorios hermanos, siempre desarrollados juntos**: [`Reaxt`](https://github.com/aon-co-jp/Reaxt), **RPoRea** y [`open-cosmo`](https://github.com/aon-co-jp/open-cosmo) (la contrapartida de código abierto en Rust de WunderGraph Cosmo). Las reglas y el estado de los puntos de contacto están en [`docs/THREE-SISTERS.md`](docs/THREE-SISTERS.md). Colócalos, junto con el repositorio padre [`RPoem`](https://github.com/aon-co-jp/RPoem), en carpetas hermanas (dependencias por ruta).

**Estado actual, con franqueza.** Funciona: los puntos de entrada, la ABI de C y el enlace con Python (comprobados en un Windows real y en Linux), las pruebas. Sin empezar o sin verificar: la composición con el enrutador de open-cosmo, Mojo/RMojo, Mac/móviles/supercomputadoras, una CLI que automatice `.ts` → JS → recursos, el despliegue. Licencia: Apache-2.0 OR MIT.

---

## Português (Brasil)

**O RPoRea une o RPoem e o Reaxt.** O RPoem é um conjunto de peças de servidor em Rust; com o Reaxt, tanto a interface quanto o servidor são escritos apenas em Rust. Com uma única chamada, um aplicativo Reaxt roda em cima do RPoem: `rporea::serve(app, addr)` (já inclui rastreamento e compressão; use `rporea::start` se precisar do endereço realmente vinculado e do handle da tarefa).

**De quais linguagens dá para usar?** O RPoRea foi pensado para ser usado **direto do Rust**. As outras linguagens chamam as mesmas funções por meio de uma biblioteca compartilhada com ABI C (`rporea-ffi`, cabeçalho `include/rporea.h`): Rust — funciona e tem testes · Python — **verificado no Windows e no Linux (Ubuntu no WSL)**, o `bindings/python/rporea.py` usa só o `ctypes` da biblioteca padrão · Mojo / RMojo — **ainda não verificado** (a máquina de desenvolvimento não tem Mojo) · C/C++ — cabeçalho incluído. Mais detalhes em [`docs/BINDINGS.md`](docs/BINDINGS.md).

**Sistemas compatíveis.** Por enquanto, Windows e Linux. Quando a versão estiver pronta, virão Mac, Android, iPhone, iPad e supercomputadores (nada disso foi iniciado).

**Use junto com o nosso HTML5 + CSS3 + TypeScript em Rust, ainda em desenvolvimento.** O RPoRea e o RPoem foram pensados para criar sites, aplicativos web e aplicativos em conjunto com as nossas versões em Rust de HTML5, CSS3 e TypeScript (`rhtml5`, `rcss3`, `RS-TypeScript`; o motor de JavaScript `RS-JavaScript` é, por ora, apenas um esqueleto). Quatro princípios: (1) usar o mínimo possível de HTML5 e CSS3; (2) usar tokio (assíncrono no servidor) e WASM (dentro do navegador) onde cada um faz sentido; (3) sem REST — Connect a partir do navegador e gRPC entre serviços; (4) um JavaScript em Rust também está em andamento, mas **o TypeScript em Rust vem primeiro**. Detalhes em [`docs/WEB-STACK.md`](docs/WEB-STACK.md).

**Três repositórios irmãos, sempre desenvolvidos juntos**: [`Reaxt`](https://github.com/aon-co-jp/Reaxt), **RPoRea** e [`open-cosmo`](https://github.com/aon-co-jp/open-cosmo) (a contrapartida open source em Rust do WunderGraph Cosmo). As regras e o estado dos pontos de contato estão em [`docs/THREE-SISTERS.md`](docs/THREE-SISTERS.md). Deixe os três, junto com o repositório pai [`RPoem`](https://github.com/aon-co-jp/RPoem), em pastas irmãs (dependências por caminho).

**Situação atual, com sinceridade.** Funciona: os pontos de entrada, a ABI C e a ligação com Python (conferidas em um Windows de verdade e no Linux), os testes. Não iniciado ou não verificado: a composição com o roteador do open-cosmo, Mojo/RMojo, Mac/celulares/supercomputadores, uma CLI que automatize `.ts` → JS → recursos, a implantação. Licença: Apache-2.0 OR MIT.

---

## Русский

**RPoRea объединяет RPoem и Reaxt.** RPoem — это набор серверных компонентов на Rust, а Reaxt позволяет писать и интерфейс, и сервер только на Rust. Одним вызовом приложение на Reaxt запускается поверх RPoem: `rporea::serve(app, addr)` (трассировка и сжатие уже включены; если нужен фактический адрес привязки и дескриптор задачи, используйте `rporea::start`).

**Из каких языков можно пользоваться?** RPoRea рассчитан на использование **напрямую из Rust**. Другие языки вызывают те же функции через общую библиотеку с C ABI (`rporea-ffi`, заголовок `include/rporea.h`): Rust — работает, есть тесты · Python — **проверено в Windows и Linux (Ubuntu в WSL)**, `bindings/python/rporea.py` использует только `ctypes` из стандартной библиотеки · Mojo / RMojo — **пока не проверено** (на машине разработки нет Mojo) · C/C++ — заголовок есть. Подробнее — в [`docs/BINDINGS.md`](docs/BINDINGS.md).

**Поддерживаемые системы.** Сейчас — Windows и Linux. Когда релиз будет готов, добавим Mac, Android, iPhone, iPad и суперкомпьютеры (пока ничего из этого не начато).

**Использование вместе с разрабатываемыми нами HTML5 + CSS3 + TypeScript на Rust.** RPoRea и RPoem задуманы для создания сайтов, веб-приложений и приложений вместе с нашими версиями HTML5, CSS3 и TypeScript на Rust (`rhtml5`, `rcss3`, `RS-TypeScript`; движок JavaScript `RS-JavaScript` пока лишь заготовка). Четыре принципа: (1) использовать как можно меньше HTML5 и CSS3; (2) применять tokio (асинхронность на сервере) и WASM (в браузере) там, где каждый из них уместен; (3) без REST — Connect из браузера и gRPC между сервисами; (4) JavaScript на Rust тоже разрабатывается, но **TypeScript на Rust в приоритете**. Подробности — в [`docs/WEB-STACK.md`](docs/WEB-STACK.md).

**Три репозитория-сестры, которые всегда развиваются вместе**: [`Reaxt`](https://github.com/aon-co-jp/Reaxt), **RPoRea** и [`open-cosmo`](https://github.com/aon-co-jp/open-cosmo) (открытый аналог WunderGraph Cosmo на Rust). Правила и текущее состояние точек стыковки — в [`docs/THREE-SISTERS.md`](docs/THREE-SISTERS.md). Держите их вместе с родительским [`RPoem`](https://github.com/aon-co-jp/RPoem) в соседних каталогах (зависимости по путям).

**Текущее состояние — честно.** Работает: точки входа, C ABI и привязка к Python (проверены на настоящем Windows и в Linux), тесты. Не начато или не проверено: совместная работа с маршрутизатором open-cosmo, Mojo/RMojo, Mac/мобильные устройства/суперкомпьютеры, CLI для автоматизации цепочки `.ts` → JS → ресурсы, развёртывание. Лицензия: Apache-2.0 OR MIT.
