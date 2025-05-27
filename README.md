# oreuit

- **oreuit** は、指定ディレクトリ配下のファイル構成・内容をまとめたテキストレポートを生成するツールです（[uithub](https://uithub.com/)インスパイア）。
- GitHub等に未登録のコードベースでも、ディレクトリツリーやファイル内容を一括で可視化できます。

# インストール

## 必要要件

- [Rust](https://www.rust-lang.org/)（1.70以上推奨）

## インストール手順

1.  **リポジトリのクローンまたはコードの取得**
    ```bash
    git clone https://github.com/yuki-inaho/uithub_like_text_generator.git
    cd uithub_like_text_generator
    ```
2.  **ビルド**
    ```bash
    # 標準ビルド（クリップボード機能なし）
    cargo build --release
    # クリップボード機能付きビルド（-cオプション利用時）
    # cargo build --release --features clipboard
    ```
    -c, --clipboard オプションを使う場合は2行目を有効化してください。
3.  **実行ファイルの配置**
    ビルド後、`./target/release/oreuit` が生成されます。PATHを通すか、直接パス指定で利用してください。

# Usage

コマンド実行時に対象ディレクトリや各種オプションを指定します。主なコマンドラインオプションは以下の通りです。

## Available Options

- `-d, --directory <DIRECTORIES>`
  - カンマ区切りで探索対象ディレクトリを指定（省略時はカレントディレクトリ）。例: `-d src,tests`

- `-e, --extensions <EXTENSIONS>`
  - 許可するファイル拡張子をカンマ区切りで指定。
  - 先頭が `+,` の場合はデフォルトリストに追加（例: `-e +,.json,.vue`）。
  - そうでない場合は指定リストで上書き（例: `-e .py,.js`）。
  - 省略時のデフォルト: `.txt`, `.md`, `.py`, `.js`, `.java`, `.cpp`, `.c`, `.cs`, `.rb`, `.go`, `.rs`, `.hpp`, `.ts`, `.tsx`, `.d.ts`, `.jsx`, `.toml`
  - **注意:** `.json` はデフォルトに含まれません。必要な場合は `-e +,.json` で追加してください。
  - **注意:** 拡張子なしファイル（例: `.gitignore`, `Makefile`, `Dockerfile`, `LICENSE`, `README`, `.gitattributes`, `justfile`）は明示的に除外しない限り許可されます。

- `-i, --ignore-extensions <EXTENSIONS>`
  - 無視する拡張子をカンマ区切りで指定。例: `-i .lock,.md`
  - デフォルト:
    ```
    .bin,.zip,.tar,.gz,.7z,.rar,.exe,.dll,.so,.dylib,.a,.lib,.obj,.o,.class,.jar,.war,.ear,.ipynb,.jpg,.jpeg,.png,.gif
    ```

- `-o, --output <OUTPUT>`
  - 出力ファイル名（デフォルト: `summary.txt`）

- `-c, --clipboard`
  - ファイル出力の代わりにクリップボードへコピー（ビルド時 `--features clipboard` 必須）

- `-I, --ignore-dirs <DIRS>`
  - 無視するディレクトリ名をカンマ区切りで指定。
  - 先頭が `+,` の場合はデフォルトリストに追加（例: `--ignore-dirs +,my_temp,build2`）。
  - そうでない場合は指定リストで上書き。
  - デフォルト: `.git`, `.vscode`, `target`, `node_modules`, `__pycache__`, `.idea`, `build`, `dist`, `.ruff_cache`, `.cache`, `.tox`, `.nox`, `.pytest_cache`, `htmlcov`, `instance`, `.env`, `.venv`, `env`, `venv`, `ENV`, `site`, `.mypy_cache`, `debug` など

- `--ignore-files <FILENAMES>`
  - 無視する**ファイル名**をカンマ区切りで指定。例: `--ignore-files Cargo.lock,summary.txt_example`
  - 拡張子指定よりも優先されますが、`--whitelist-filenames` に含まれる場合は無視されません。

- `--max-size <MAX_SIZE>`
  - ファイル内容を読み込む最大サイズ（バイト単位、デフォルト: 10485760=10MB）。

- `-w, --whitelist-filenames <FILENAMES>`
  - 常に含めるファイル名をカンマ区切りで指定（例: `Dockerfile,Makefile`）。デフォルト: `Dockerfile,Makefile,justfile`

---

## Command and Output Examples

```bash
# カレントディレクトリを探索し、tempディレクトリを無視し、.jsonも対象に追加
./target/release/oreuit -d . -I +,temp -e +,.json -o summary.txt

# srcディレクトリのみ、.pyと.jsのみ対象
./target/release/oreuit -d src -e .py,.js -o summary_src.txt

# buildディレクトリのみを無視（デフォルトリストを上書き）
./target/release/oreuit -d . --ignore-dirs build -o summary_build_only.txt

# Cargo.lock, summary.txt_example というファイル名を無視してsummary.txtを生成
./target/release/oreuit --ignore-files Cargo.lock,summary.txt_example -o summary.txt
```

---

- 詳細な使い方やオプションの優先順位は `--help` も参照してください。
