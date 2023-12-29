# ディレクトリ構造  
[参考にさせて頂いたサイト](https://zenn.dev/dencyu/articles/a94928b9ce45f7)
```
app-backend/
├ src/
│ ├ assets/
│ │ └ //image-files
│ ├ crud/
│ │ ├ __init__.py
│ │ └ //crud-files
│ ├ database/
│ │ ├ seedings/
│ │ │ ├ __init__.py
│ │ │ └ //seeding-files
│ │ ├ __init__.py
│ │ ├ database.py
│ │ └ master_seeding.py
│ ├ routers/
│ │ ├ __init__.py
│ │ └ //routing-files
│ ├ schemas/
│ │ ├ __init__.py
│ │ └ //schema-files
│ ├ services/
│ │ ├ __init__.py
│ │ └ //service-files
│ ├ __init__.py
│ ├ dependencies.py
│ ├ main.py
│ └ models.py
├ tests/
│ ├ integration/
│ │ ├ __init__.py
│ │ └ //integration-test-files
│ ├ unit/
│ │ ├ __init__.py
│ │ └ //unit-test-files
│ ├ __init__.py
│ ├ conftest.py
│ └ dependencies.py
├ (.env)
├ .env.sample
└ .gitignore
# ()内表記は.gitignore対象.
# /で終わるものはディレクトリを表す.
# //で始まる表記は追加するファイルの内容を表す.
```
