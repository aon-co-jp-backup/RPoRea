# RPoRea

[日本語](#日本語) · [English](#english) · [简体中文](#简体中文) · [繁體中文(台灣)](#繁體中文台灣) · [한국어](#한국어) · [Deutsch](#deutsch) · [Français](#français) · [Español](#español) · [Português (Brasil)](#português-brasil) · [Русский](#русский)

> English, 中文, 한국어, Deutsch, Français, Español, Português, Русский は AI 翻訳です。ネイティブの確認はまだです。気づいた点は Issue へ。

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

---

## 日本語

**RPoRea は、RPoem と Reaxt を1つにしたものです。** Rust だけで、画面もサーバーも作れます。起動は1行です。

- **使える言語**: 基本は Rust です。Python も使えます(Windows と Linux で動作を確認しました)。Mojo と RMojo はまだ試していません。
- **対応 OS**: 今は Windows と Linux。完成したら Mac・Android・iPhone・iPad・スーパーコンピューターにも広げます。
- **一緒に使うもの**: 開発中の Rust 版 HTML5・CSS3・TypeScript です。決めごとは4つ。HTML と CSS はできるだけ少なく。tokio と WASM は合うところだけ。REST は使わない。JavaScript より TypeScript を先に作る。

**いまの状態**: 動くもの=起動の入口、Python 連携、テスト。まだのもの=open-cosmo との結合確認、Mojo/RMojo、Mac やスマホ。

詳しくは [docs/BINDINGS.md](docs/BINDINGS.md) · [docs/WEB-STACK.md](docs/WEB-STACK.md) · [docs/THREE-SISTERS.md](docs/THREE-SISTERS.md)

---

## English

**RPoRea joins RPoem and Reaxt.** You can build both the screen and the server in Rust alone. Starting an app takes one line.

- **Languages**: Rust is the main one. Python works too (checked on Windows and Linux). Mojo and RMojo are not tested yet.
- **Systems**: Windows and Linux for now. Mac, Android, iPhone, iPad and supercomputers come after the release is done.
- **Works with**: our Rust versions of HTML5, CSS3 and TypeScript, which are still being built. Four rules: use as little HTML and CSS as possible; use tokio and WASM only where they fit; no REST; build TypeScript before JavaScript.

**Status**: working: the start-up entry point, Python support, tests. Not yet: joining with open-cosmo, Mojo/RMojo, Mac and phones.

More: [docs/BINDINGS.md](docs/BINDINGS.md) · [docs/WEB-STACK.md](docs/WEB-STACK.md) · [docs/THREE-SISTERS.md](docs/THREE-SISTERS.md)

---

## 简体中文

**RPoRea 把 RPoem 和 Reaxt 合在了一起。** 只用 Rust,就能做出界面和服务器。启动只要一行代码。

- **可用的语言**:主要是 Rust。Python 也能用(已在 Windows 和 Linux 上确认)。Mojo 和 RMojo 还没有试过。
- **支持的系统**:现在是 Windows 和 Linux。正式版完成后,再支持 Mac、Android、iPhone、iPad 和超级计算机。
- **搭配使用**:我们正在开发的 Rust 版 HTML5、CSS3 和 TypeScript。有四条约定:HTML 和 CSS 尽量少用;tokio 和 WASM 只在合适的地方用;不用 REST;先做 TypeScript,再做 JavaScript。

**现状**:能用的有启动入口、Python 支持和测试。还没做的有与 open-cosmo 的组合验证、Mojo/RMojo、Mac 和手机。

详情见 [docs/BINDINGS.md](docs/BINDINGS.md) · [docs/WEB-STACK.md](docs/WEB-STACK.md) · [docs/THREE-SISTERS.md](docs/THREE-SISTERS.md)

---

## 繁體中文(台灣)

**RPoRea 把 RPoem 和 Reaxt 合在一起。** 只用 Rust,就能做出畫面和伺服器。啟動只要一行程式碼。

- **可用的語言**:主要是 Rust。Python 也能用(已在 Windows 和 Linux 上確認)。Mojo 和 RMojo 還沒試過。
- **支援的系統**:現在是 Windows 和 Linux。正式版完成後,再支援 Mac、Android、iPhone、iPad 和超級電腦。
- **搭配使用**:我們正在開發的 Rust 版 HTML5、CSS3 和 TypeScript。有四項約定:HTML 和 CSS 盡量少用;tokio 和 WASM 只用在合適的地方;不用 REST;先做 TypeScript,再做 JavaScript。

**現況**:能用的有啟動入口、Python 支援和測試。還沒做的有與 open-cosmo 的整合驗證、Mojo/RMojo、Mac 和手機。

詳情請見 [docs/BINDINGS.md](docs/BINDINGS.md) · [docs/WEB-STACK.md](docs/WEB-STACK.md) · [docs/THREE-SISTERS.md](docs/THREE-SISTERS.md)

---

## 한국어

**RPoRea는 RPoem과 Reaxt를 하나로 합친 것입니다.** Rust만으로 화면도 서버도 만들 수 있습니다. 시작은 코드 한 줄이면 됩니다.

- **쓸 수 있는 언어**: 기본은 Rust입니다. Python도 쓸 수 있습니다(Windows와 Linux에서 확인했습니다). Mojo와 RMojo는 아직 해 보지 않았습니다.
- **지원 OS**: 지금은 Windows와 Linux입니다. 완성되면 Mac, Android, iPhone, iPad, 슈퍼컴퓨터로 넓힙니다.
- **함께 쓰는 것**: 개발 중인 Rust판 HTML5, CSS3, TypeScript입니다. 약속은 네 가지입니다. HTML과 CSS는 가능한 한 적게. tokio와 WASM은 어울리는 곳에만. REST는 쓰지 않기. JavaScript보다 TypeScript를 먼저 만들기.

**현재 상태**: 되는 것은 시작 진입점, Python 연동, 테스트입니다. 아직인 것은 open-cosmo와의 결합 확인, Mojo/RMojo, Mac과 스마트폰입니다.

자세한 내용은 [docs/BINDINGS.md](docs/BINDINGS.md) · [docs/WEB-STACK.md](docs/WEB-STACK.md) · [docs/THREE-SISTERS.md](docs/THREE-SISTERS.md)

---

## Deutsch

**RPoRea verbindet RPoem und Reaxt.** Oberfläche und Server entstehen allein mit Rust. Ein Programm startet mit einer Zeile.

- **Sprachen**: Hauptsprache ist Rust. Python geht auch (auf Windows und Linux geprüft). Mojo und RMojo sind noch nicht getestet.
- **Systeme**: Zurzeit Windows und Linux. Mac, Android, iPhone, iPad und Supercomputer folgen nach der fertigen Version.
- **Passt zusammen mit**: unserem Rust-HTML5, -CSS3 und -TypeScript, die noch in Arbeit sind. Vier Regeln: so wenig HTML und CSS wie möglich; tokio und WASM nur, wo sie passen; kein REST; erst TypeScript, dann JavaScript.

**Stand**: Es läuft: der Startpunkt, die Python-Anbindung, die Tests. Noch offen: das Zusammenspiel mit open-cosmo, Mojo/RMojo, Mac und Handys.

Mehr dazu: [docs/BINDINGS.md](docs/BINDINGS.md) · [docs/WEB-STACK.md](docs/WEB-STACK.md) · [docs/THREE-SISTERS.md](docs/THREE-SISTERS.md)

---

## Français

**RPoRea réunit RPoem et Reaxt.** On crée l'écran et le serveur avec Rust seulement. Pour démarrer, une seule ligne suffit.

- **Langages** : Rust est le principal. Python marche aussi (vérifié sous Windows et Linux). Mojo et RMojo ne sont pas encore testés.
- **Systèmes** : Windows et Linux pour l'instant. Mac, Android, iPhone, iPad et supercalculateurs viendront ensuite.
- **À utiliser avec** : nos versions Rust de HTML5, CSS3 et TypeScript, encore en cours de création. Quatre règles : le moins possible de HTML et de CSS ; tokio et WASM seulement là où ils conviennent ; pas de REST ; TypeScript avant JavaScript.

**État** : ça marche : le point de départ, le lien avec Python, les tests. Pas encore : l'assemblage avec open-cosmo, Mojo/RMojo, Mac et téléphones.

Pour en savoir plus : [docs/BINDINGS.md](docs/BINDINGS.md) · [docs/WEB-STACK.md](docs/WEB-STACK.md) · [docs/THREE-SISTERS.md](docs/THREE-SISTERS.md)

---

## Español

**RPoRea une RPoem y Reaxt.** Con solo Rust se hace la pantalla y el servidor. Para empezar basta una línea.

- **Lenguajes**: el principal es Rust. Python también funciona (probado en Windows y Linux). Mojo y RMojo aún no se han probado.
- **Sistemas**: por ahora Windows y Linux. Mac, Android, iPhone, iPad y supercomputadoras llegarán cuando la versión esté lista.
- **Para usar junto con**: nuestras versiones en Rust de HTML5, CSS3 y TypeScript, que aún están en construcción. Cuatro reglas: lo menos posible de HTML y CSS; tokio y WASM solo donde encajen; sin REST; primero TypeScript y después JavaScript.

**Estado**: funciona: el punto de arranque, el enlace con Python y las pruebas. Falta: unirlo con open-cosmo, Mojo/RMojo, Mac y móviles.

Más información: [docs/BINDINGS.md](docs/BINDINGS.md) · [docs/WEB-STACK.md](docs/WEB-STACK.md) · [docs/THREE-SISTERS.md](docs/THREE-SISTERS.md)

---

## Português (Brasil)

**O RPoRea junta o RPoem e o Reaxt.** Só com Rust, você cria a tela e o servidor. Para começar, basta uma linha.

- **Linguagens**: a principal é o Rust. O Python também funciona (testado no Windows e no Linux). Mojo e RMojo ainda não foram testados.
- **Sistemas**: por enquanto, Windows e Linux. Mac, Android, iPhone, iPad e supercomputadores vêm depois que a versão estiver pronta.
- **Para usar junto com**: nossas versões em Rust de HTML5, CSS3 e TypeScript, que ainda estão sendo feitas. Quatro regras: o mínimo possível de HTML e CSS; tokio e WASM só onde fizerem sentido; sem REST; primeiro o TypeScript, depois o JavaScript.

**Situação**: já funciona: o ponto de partida, a ligação com Python e os testes. Ainda não: juntar com o open-cosmo, Mojo/RMojo, Mac e celulares.

Saiba mais: [docs/BINDINGS.md](docs/BINDINGS.md) · [docs/WEB-STACK.md](docs/WEB-STACK.md) · [docs/THREE-SISTERS.md](docs/THREE-SISTERS.md)

---

## Русский

**RPoRea объединяет RPoem и Reaxt.** Только на Rust можно сделать и экран, и сервер. Запуск — одна строка.

- **Языки**: главный — Rust. Python тоже работает (проверено в Windows и Linux). Mojo и RMojo пока не проверялись.
- **Системы**: сейчас Windows и Linux. Mac, Android, iPhone, iPad и суперкомпьютеры — когда будет готов релиз.
- **Работает вместе с**: нашими версиями HTML5, CSS3 и TypeScript на Rust, которые ещё в разработке. Четыре правила: как можно меньше HTML и CSS; tokio и WASM — только там, где они нужны; без REST; сначала TypeScript, потом JavaScript.

**Состояние**: работает: точка запуска, связь с Python, тесты. Ещё нет: связка с open-cosmo, Mojo/RMojo, Mac и телефоны.

Подробнее: [docs/BINDINGS.md](docs/BINDINGS.md) · [docs/WEB-STACK.md](docs/WEB-STACK.md) · [docs/THREE-SISTERS.md](docs/THREE-SISTERS.md)

---

Apache-2.0 OR MIT
