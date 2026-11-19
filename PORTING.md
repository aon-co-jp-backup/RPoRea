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
