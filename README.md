# discord-cache-cleaner

Discordのキャッシュを削除するCLI

## Todo

- [ ] Windows のサポート
- [ ] Linux のサポート
- [ ] 複数の Discord インスタンスのサポート
  - [ ] Discord Canary のサポート
  - [ ] Discord PTB のサポート
  - [ ] Discord Development のサポート

## Installation

### Using cargo

```shell
cargo install discord-cache-cleaner
```
[Rust のインストール](https://doc.rust-jp.rs/book-ja/ch01-01-installation.html)が必要です。

## Usage

```shell
discord-cache-cleaner
```

- 実行したOSに応じて、Discordのキャッシュを削除します。
- キャッシュを削除した後は `Ctrl + R` でDiscordインスタンスを再起動してください。

### Other instance

```shell
discord-cache-cleaner --instance [instance-type]
```

指定したDiscordインスタンスのキャッシュを削除します。

`[instance-type]` には以下のいずれかを指定してください。

- `1`: Discord Stable (安定版)
- `2`: Discord Canary (アルファ版)
- `3`: Discord PTB (公開テスト版ビルド)
- `4`: Discord Development (開発版)

指定しない場合は Discord Stable (安定版) のキャッシュを削除します。
