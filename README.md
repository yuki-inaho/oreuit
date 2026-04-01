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
  - 省略時のデフォルト: `.txt`, `.md`, `.py`, `.js`, `.java`, `.cpp`, `.c`, `.cs`, `.rb`, `.go`, `.rs`, `.hpp`, `.ts`, `.tsx`, `.d.ts`, `.jsx`, `.toml`, `.msg`, `.srv`, `.action`, `.launch`, `.urdf`, `.xacro`, `.cfg`
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

- `--config <CONFIG>`
  - whitelist / blacklist を定義した TOML 設定ファイルを読み込みます。
  - `--config` 指定時、フィルタ条件は **config 側に完全切替** されます。
  - つまり `--extensions` / `--ignore-extensions` / `--ignore-dirs` / `--ignore-files` / `--whitelist-filenames` とは暗黙 merge されません。
  - 一方で `-d, --directory`、`-o, --output`、`--max-size`、`-c, --clipboard` は通常どおり有効です。

- `--generate-config`
  - 現在のデフォルト設定相当の TOML を標準出力へ出力して終了します。
  - このオプションは早期終了し、探索や出力ファイル書き込みは行いません。

## TOML Configuration

`--generate-config` でテンプレートを生成し、そのまま編集して `--config` に渡せます。

```toml
[whitelist]
extensions = [".rs", ".py"]
files = ["Dockerfile", "Makefile", "justfile"]

[blacklist]
extensions = [".png", ".jpg"]
files = ["Cargo.lock"]
directories = [".git", "target", "node_modules"]
```

各フィールドの意味:

- `whitelist.extensions`
  - 含める拡張子一覧です。
  - `rs`, `.rs`, ` RS ` のような表記ゆれは内部で正規化され、同じ意味として扱われます。

- `whitelist.files`
  - 常に含めるファイル名一覧です。
  - `blacklist.files` より優先されます。

- `blacklist.extensions`
  - 除外する拡張子一覧です。

- `blacklist.files`
  - 除外するファイル名一覧です。

- `blacklist.directories`
  - 再帰探索と tree 表示の両方から除外するディレクトリ名一覧です。

### Precedence and Behavior

- `--config` を指定した場合、フィルタ設定は TOML のみを使います。CLI のフィルタ系オプションとは混ざりません。
- `whitelist.files` は `blacklist.files` より優先されます。
- 拡張子なしファイルは、既定 CLI 出力との互換性を保つため、`Dockerfile`, `Makefile`, `LICENSE`, `README`, `.gitignore`, `.gitattributes`, `justfile` を既定で扱います。
- `--generate-config` が出力する `whitelist.files` は `Dockerfile`, `Makefile`, `justfile` ですが、config 経路でも既定の extensionless 挙動は維持されます。

### Error Behavior

- 存在しない config パスを指定した場合:
  - `Config file not found or unreadable: ...`
- TOML 構文が不正な場合:
  - `Config TOML parse error: ...`

missing path と parse error は別メッセージで区別されます。

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

# デフォルト相当の設定テンプレートを生成して保存
./target/release/oreuit --generate-config > oreuit.toml

# 生成した TOML を編集して適用
./target/release/oreuit --config oreuit.toml -d . -o summary_from_config.txt

# config 指定時でも出力先や対象ディレクトリは CLI で指定できる
./target/release/oreuit --config oreuit.toml -d src,tests --max-size 2097152 -o summary_filtered.txt

# 不正 TOML の挙動を確認
./target/release/oreuit --config broken.toml -d .
```

---

- 詳細な使い方やオプションの優先順位は `--help` も参照してください。
