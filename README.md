# Rustlings-jp 🦀-🇯🇵

Rustlingsの**日本語版サードパーティー**へようこそ😃
不備などございましたら、[こちら](https://github.com/sotanengel/rustlings-jp/issues)から連絡くださいませ。


# 導入方法
※ 仕様の更新などで手順に変更が加わる可能性があるため、問題が生じた場合には[本家](https://github.com/rust-lang/rustlings/blob/main)などを確認し、
[こちら](https://github.com/sotanengel/rustlings-jp/issues)で修正内容を報告いただけますと幸いです。


## 1. Rustをインストールする
まず[www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)にアクセスし、最新バージョンのRustをインストールしましょう。

> 🐧 もしもLinuxであれば, `gcc`をインストールしましょう。
>
> Deb: `sudo apt install gcc`.
> Dnf: `sudo dnf install gcc`.

> 🍎 もしもMacOSを使っている場合は,Xcodeとデベロッパーツールを`xcode-select --install`でインストールしてください。


## 2. Rustlingsをインストールする
Rustlingsの演習をスムーズに進めるためのコマンドラインツールをターミナル上からインストールします。


```bash
cargo install rustlings
```

<details>
<summary><strong>もしも失敗した場合には…</strong> (<em>詳細を開く</em>)</summary>

- `rustup update`コマンドで最新バージョンのRustか確認してください
- `--locked`フラグを利用してみてください、こんな感じで→`cargo install rustlings --locked`
- もしくは[本家のissue](https://github.com/rust-lang/rustlings/issues/new)で報告してください

</details>


## 3.演習問題のダウンロード
最新バージョンの日本語の演習問題をダウンロードしましょう！
ダウンロードしたzipファイルを解凍し、好きな場所に配置してください。

[ダウンロード](https://github.com/sotanengel/rustlings-jp/tree/20240829)

## 4. 演習問題にチャレンジ！
ターミナル上で解凍した`rustlings-jp-YYYYMMDD`のディレクトリに移動し、以下のコマンドを実行してください。


```bash
rustlings
```


<details>
<summary><strong>もしも「<code>rustlings</code>コマンドが見つからない」とターミナル上で警告が出た場合には…</strong> (<em>詳細を開く</em>)</summary>


もしもLinux系を使っており、Rustをパッケージマネージャーでインストールしていた場合には、
Cargoが`~/.cargo/bin`にダウンロードされているものの、
 `~/.cargo/bin`が`PATH`の環境変数に入っていない可能性があります。

解決方法としては、

- 手動で`~/.cargo/bin`を`PATH`に追加する
- Rustをアンインストールし、`rustup`: https://www.rust-lang.org/tools/install でインストールする

</details>


# 操作方法の簡単な説明
- `rustlings`：問題集を解くためのツールが起動する
- `n`：次の問題に進む
- `l`：問題のリスト一覧を表示する
  - `c`：カーソルで合わせた問題から演習を再開する
- `r`：問題の回答ステータスをリセットする
