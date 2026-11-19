# RPoRea お引越し・再開

兄弟ディレクトリに `Reaxt` `RPoem` `open-cosmo` `open-web-server`(RPoem が依存)を clone してから始める。

## 日本語

1. ビルド: `cargo build -p rporea-ffi`
2. 試験: `py -3 bindings/python/test_rporea.py`(`python binding OK` が出れば成功)
3. 次の作業: open-cosmo のルーターに Reaxt のサブグラフを合成して確認する。

## English

1. Build: `cargo build -p rporea-ffi`
2. Test: `py -3 bindings/python/test_rporea.py` (success prints `python binding OK`)
3. Next: join Reaxt's subgraph with the open-cosmo router and check it.

## 简体中文

1. 构建:`cargo build -p rporea-ffi`
2. 测试:`py -3 bindings/python/test_rporea.py`(成功时会显示 `python binding OK`)
3. 下一步:把 Reaxt 的子图接到 open-cosmo 路由器上并验证。

## 繁體中文(台灣)

1. 建置:`cargo build -p rporea-ffi`
2. 測試:`py -3 bindings/python/test_rporea.py`(成功時會顯示 `python binding OK`)
3. 下一步:把 Reaxt 的子圖接到 open-cosmo 路由器上並驗證。

## 한국어

1. 빌드: `cargo build -p rporea-ffi`
2. 시험: `py -3 bindings/python/test_rporea.py` (성공하면 `python binding OK`가 나옵니다)
3. 다음 작업: Reaxt의 서브그래프를 open-cosmo 라우터에 붙여서 확인합니다.

## Deutsch

1. Bauen: `cargo build -p rporea-ffi`
2. Testen: `py -3 bindings/python/test_rporea.py` (bei Erfolg erscheint `python binding OK`)
3. Als Nächstes: den Subgraph von Reaxt mit dem open-cosmo-Router zusammenbringen und prüfen.

## Français

1. Compiler : `cargo build -p rporea-ffi`
2. Tester : `py -3 bindings/python/test_rporea.py` (en cas de succès : `python binding OK`)
3. Ensuite : relier le sous-graphe de Reaxt au routeur open-cosmo et le vérifier.

## Español

1. Compilar: `cargo build -p rporea-ffi`
2. Probar: `py -3 bindings/python/test_rporea.py` (si sale bien, muestra `python binding OK`)
3. Siguiente: unir el subgrafo de Reaxt con el enrutador de open-cosmo y comprobarlo.

## Português (Brasil)

1. Compilar: `cargo build -p rporea-ffi`
2. Testar: `py -3 bindings/python/test_rporea.py` (se der certo, aparece `python binding OK`)
3. Próximo passo: juntar o subgrafo do Reaxt ao roteador do open-cosmo e conferir.

## Русский

1. Сборка: `cargo build -p rporea-ffi`
2. Проверка: `py -3 bindings/python/test_rporea.py` (при успехе выводится `python binding OK`)
3. Дальше: подключить подграф Reaxt к маршрутизатору open-cosmo и проверить.
