# nos_pico

A project for performing Nostr digital signatures on a Raspberry Pi Pico and sending them to relay servers via Rust.

## Overview

This project aims to perform signing operations securely inside external hardware (RP2040) without ever loading the private key into PC memory.

It is, in essence, a hardware version of NIP-07.

- **Pico (C++):** Receives data over serial communication, performs Schnorr signatures and other digital signing operations, and returns the result.
- **Host (Rust):** Handles the Nostr protocol, manages serial communication with the Pico, and manages WebSocket communication with relay servers.

## Project Structure

```text
NOS_PICO
├── nos_pico_hard/      # Hardware-side Pico SDK (C++) project
│   ├── nos_pico_hard.cpp  # Main logic (serial wait & processing)
│   ├── nostr_sign.cpp     # Signature library calls
│   └── CMakeLists.txt     # Build configuration
└── nos_pico_rust/      # Host-side (Rust) project
    ├── src/               # Rust source code
    └── Cargo.toml         # Dependencies (serialport, tokio-tungstenite, etc.)
```

## Build Instructions

First clone the repository with `git clone --recursive https://github.com/moyashi170607/nos_pico.git`, then follow the steps below.

### RP2040
1. Change directory: `cd nos_pico_hard`
2. Create the config file: `touch config.hpp`
3. Edit `config.hpp` as follows:

```
#ifndef CONFIG_NOSTR_KEY
#define CONFIG_NOSTR_KEY 1

#include <stdint.h>

const char* seckey_hex = "your private key in hex format";

#endif  // !CONFIG_NOSTR_KEY
```

4. Build using the Pico SDK.
5. Write the generated `build/nos_pico_hard.uf2` to the Raspberry Pi Pico.

### Host side (Rust)
1. Change directory: `cd nos_pico_rust`
2. Build: `cargo build --release`
3. The executable `target/release/nos_pico_rust.exe` will be generated.

## Usage
1. Connect the Raspberry Pi Pico flashed with the `nos_pico_hard` firmware.
2. Create `config.toml` in the same directory as `nos_pico_rust.exe`.
3. Edit `config.toml` as follows:
```
relays = [
    "wss link of relay server to connect",
    "wss link of relay server to connect 2",
    "wss link of relay server to connect 3",
]

port_name = "name of the port your microcontroller is connected to"
```

4. Run `nos_pico_rust.exe` in a terminal.
5. After connecting to relay servers, your public key will be displayed and you will be prompted with `Please enter your post content`.
6. Enter the content you want to post.
7. The message will be signed with the private key configured on the microcontroller and sent to the relays.
8. You can continue posting by entering new content each time.

## Contributing
Issues and PRs are both welcome.

## Acknowledgements
This project makes use of various libraries including the Pico SDK, secp256k1 by the Bitcoin Core development team, and various Rust crates.
We are grateful to their developers and to the Raspberry Pi Foundation.

## License / Disclaimer
This project is released under the **MIT License**. See `LICENSE` for details.

This project handles Nostr private keys, which require an extremely high level of confidentiality.
**Use this project at your own risk.**
The author assumes no responsibility in the event of any key leakage or other incidents.

---

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