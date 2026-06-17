# 42Header for Zed

Zedで42 Headerを挿入・更新するための外部フォーマッタです。

VS Code版のような「任意コマンドでバッファを書き換える拡張API」は、現時点のZed拡張ではまだ限定的です。その代わり、Zedの外部フォーマッタ機能を使います。Zedが現在のバッファをstdinでこのCLIに渡し、stdoutの内容をエディタへ反映します。

## Features

- 42 Headerの挿入
- 既存ヘッダーの `Updated` とファイル名の更新
- 既存ヘッダーの `Created` 情報の保持
- Zedの `format_on_save` による保存時更新
- macOS, Linux, Windows対応のRust CLI

## Install

GitHubから直接インストール:

```sh
cargo install --git https://github.com/yoshi-bbb/42header-zed
```

ローカルで開発・確認する場合:

```sh
cargo install --path .
```

ZedをDockやランチャーから起動していて `zed-42header` が見つからない場合は、`command` に `~/.cargo/bin/zed-42header` のような絶対パスを指定してください。

## Zed Settings

`zed: open settings file` で `settings.json` を開き、必要な言語に外部フォーマッタを追加します。

```json
{
  "languages": {
    "C": {
      "formatter": {
        "external": {
          "command": "zed-42header",
          "arguments": ["--stdin-filepath", "{buffer_path}", "--language", "C"]
        }
      },
      "format_on_save": "on"
    },
    "C++": {
      "formatter": {
        "external": {
          "command": "zed-42header",
          "arguments": ["--stdin-filepath", "{buffer_path}", "--language", "C++"]
        }
      },
      "format_on_save": "on"
    }
  }
}
```

既存のフォーマッタと併用する場合は、配列で最後に `zed-42header` を置きます。

```json
{
  "languages": {
    "C": {
      "formatter": [
        { "language_server": { "name": "clangd" } },
        {
          "external": {
            "command": "zed-42header",
            "arguments": ["--stdin-filepath", "{buffer_path}", "--language", "C"]
          }
        }
      ],
      "format_on_save": "on"
    }
  }
}
```

ユーザー名とメールアドレスは引数か環境変数で指定できます。

```json
{
  "languages": {
    "C": {
      "formatter": {
        "external": {
          "command": "zed-42header",
          "arguments": [
            "--stdin-filepath",
            "{buffer_path}",
            "--language",
            "C",
            "--user",
            "marvin",
            "--email",
            "marvin@student.42.fr"
          ]
        }
      }
    }
  }
}
```

環境変数を使う場合:

```sh
export FT_HEADER_USERNAME=marvin
export FT_HEADER_EMAIL=marvin@student.42.fr
```

## Usage

Zedで `editor: format` を実行すると、ヘッダーがないファイルには挿入し、既存の42 Headerがあるファイルでは `Updated` とファイル名を更新します。`format_on_save` が `on` の言語では保存時にも更新されます。

CLIとして直接使うこともできます。

```sh
zed-42header --stdin-filepath main.c < main.c > /tmp/main.c
```

ファイルを直接書き換える場合:

```sh
zed-42header --write --stdin-filepath main.c
```

## Supported Comment Styles

パスまたは `--language` からコメントスタイルを推定します。必要なら `--comment-style slash` のように明示できます。

- `slash`: C, C++, Java, JavaScript, TypeScript, Go, Rust など
- `hash`: Python, Ruby, Shell Script, Makefile, Dockerfile, YAML など
- `semicolon`: INI
- `paren`: OCaml, F#
- `dash`: Lua, Haskell
- `percent`: LaTeX

## Notes

Zed公式ドキュメントでは、外部フォーマッタはstdinを読みstdoutへ整形後のテキストを書く形式で、`{buffer_path}` を引数に渡せます。複数フォーマッタも配列で連続実行できます。

Zedの公式拡張レジストリへ公開する場合は、`extension.toml` を持つZed拡張として `zed-industries/extensions` にPRする必要があります。このプロジェクトは現時点ではZed拡張ではなく、Zedから呼び出す外部フォーマッタCLIです。

- https://zed.dev/docs/reference/all-settings#formatter
- https://zed.dev/docs/extensions/developing-extensions

## Contributing

開発手順は [CONTRIBUTING.md](CONTRIBUTING.md) を参照してください。

公開手順は [docs/PUBLISHING.md](docs/PUBLISHING.md) にまとめています。

## License

MIT

This project is inspired by the MIT-licensed VS Code 42 Header extension. See [NOTICE.md](NOTICE.md).
