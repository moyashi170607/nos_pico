# nos_pico

Raspberry Pi PicoでNostrの電子署名を行い、Rust経由でリレーサーバーに送信するためのプロジェクトです。

## 概要

このプロジェクトは、秘密鍵をPC上のメモリに展開せず、外部ハードウェア（RP2040）内で安全に署名処理を行うことを目的としています。

いわばNIP-07のハードウェア版です。

- **Pico (C++):** シリアル通信で受け取ったデータに対して、Schnorr署名等の電子署名を行い結果を返します。
- **Host (Rust):** Nostrプロトコルを扱い、Picoとのシリアル通信およびリレーサーバーとのWebSocket通信を管理します。

## プロジェクト構造

```text
NOS_PICO
├── nos_pico_hard/      # ハードウェア側Pico SDK (C++) プロジェクト
│   ├── nos_pico_hard.cpp  # メインロジック（シリアル待機・処理）
│   ├── nostr_sign.cpp     # 署名ライブラリの呼び出し
│   └── CMakeLists.txt     # ビルド設定
└── nos_pico_rust/      # ホスト側 (Rust) プロジェクト
    ├── src/               # Rustソースコード
    └── Cargo.toml         # 依存関係（serialport, tokio-tungstenite等）
```

## ビルド手順

`git clone --recursive https://github.com/moyashi170607/nos_pico.git`でソースコードを取得した後、以下の手順でビルドを行う。

### RP2040
1. `cd nos_pico_hard`でディレクトリを移動
2. `touch config.hpp`で`config.hpp`を作成
3. `config.hpp`の内部を以下のように書き換える

```
#ifndef CONFIG_NOSTR_KEY
#define CONFIG_NOSTR_KEY 1

#include <stdint.h>

const char* seckey_hex = "あなたの秘密鍵をhex形式で書く";

#endif  // !CONFIG_NOSTR_KEY
```

4. picoSDKを用いてビルドする。
5. 生成された`build/nos_pico_hard.uf2`をRaspberry Pi Picoに書き込む

### ホスト側(Rust)
1. `cd nos_pico_rust`でディレクトリを移動
2. `cargo build --release`でビルド
3. `target/release/nos_pico_rust.exe`が生成される

## 使い方
1. `nos_pico_hard`のビルド結果を書き込んだRaspberry Pi Picoを接続
2. `nos_pico_rust.exe`と同じ階層のディレクトリに`config.toml`を作成
3. `config.toml`を以下のように書き換える
```
relays = [
    "接続するリレーサーバーのwssリンク",
    "接続するリレーサーバーのwssリンク2",
    "接続するリレーサーバーのwssリンク3",

]

port_name = "マイコンを接続しているポートの名前"

```

4. `nos_pico_rust.exe`をターミナルで実行する
5. リレーサーバーとの接続等が終わり、公開鍵が表示、`投稿内容を入力してください`の表示が出る
6. 投稿したい内容を入力
7. マイコンで設定した秘密鍵で署名後、リレーに送信される
8. その後も、投稿した内容を入力するごとに投稿される。

## コントリビュート
issue, PR どちらも大歓迎です。

## 謝辞
本プロジェクトでは、picoSDK、Bitcoin Core開発チームのsecp256k1、Rustの各種crateなど、様々なライブラリを用いております。
これらの開発者および、Raspberry Pi財団に感謝します。

## License/免責事項
本プロジェクトは**MIT License**に従って公開されています。詳しくは`LICENSE`をご覧ください。

また、本プロジェクトはNostrの秘密鍵という非常に高い機密性が求められるものを取り扱います。
本プロジェクトの利用は**自己責任で**よろしくお願いします。
万が一漏洩などがあった場合は当方は責任を負いかねます。