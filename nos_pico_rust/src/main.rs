//標準人出力
use std::io::{self, Write};

//UNIX時間取得
use tokio::sync::mpsc;

use crate::app::NostrApp;
use crate::config::{get_port_name, set_relay};
use crate::nostr::ws_send_tx::start_ws_send_tx;
use crate::serial::SerialManager;

use futures::stream::StreamExt;

//シリアル通信
mod serial;

//ノストラ関連
mod nostr;

//ユーザーが操作するアプリケーション部分
mod app;

//設定ファイル関連
mod config;

const BAUD_RATE: u32 = 9600;

// 標準入力をブロックせずに受け取るためのヘルパー関数
fn input_blocking() -> String {
    //print!("メッセージを入力: ");
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("読み込み失敗");
    buffer.trim().to_owned()
}

#[tokio::main]
async fn main() {
    let port_name: String = get_port_name();

    println!("port_name:{}", port_name);

    //接続するリレーサーバーを設定
    let relays: Vec<String> = set_relay();
    println!("relays:{:?}", relays);

    println!("署名機に接続中...");
    //接続の結果を出力
    let serial_manager = match SerialManager::open_port(&port_name, BAUD_RATE).await {
        Ok(port) => {
            println!("デバイスとの接続を確認");
            port
        }
        Err(err) => {
            eprintln!("ポートのオープンに失敗しました: {}", err);
            return;
        }
    };

    println!("リレーサーバーに接続中...");
    //リレーサーバとの通信を開始
    let (ws_writers, _ws_readers) = nostr::web_socket::open_ws(relays).await;

    //WebSocket送信のtx rxを作成
    let ws_send_tx = start_ws_send_tx(ws_writers);

    let mut app = NostrApp::new(serial_manager, ws_send_tx);

    let (input_tx, mut input_rx) = mpsc::channel::<String>(32);

    tokio::spawn(async move {
        loop {
            let msg = tokio::task::spawn_blocking(input_blocking).await.unwrap();
            if !msg.is_empty() {
                input_tx.send(msg).await.ok();
            }
        }
    });

    //公開鍵の送信をマイコンにリクエスト
    println!("公開鍵を取得中...");
    let _ = SerialManager::send_data(&mut app.serial, "get_pubkey".as_bytes()).await;

    println!("投稿内容を入力してください");

    loop {
        tokio::select! {
            // シリアルポートからデータ（署名や公開鍵）が届いたとき
            Some(line_result) = app.serial.reader.next() => {
                match line_result {
                    Ok(line) => {
                        app.handle_serial_line(line).await;
                    }
                    Err(e) => eprintln!("シリアル読込エラー: {}", e),
                }
            }

            // ユーザーがコンソールにメッセージを打ち込んだとき
            Some(user_msg) = input_rx.recv() => {
                app.handle_user_input(user_msg).await;
            }
        }
    }
}
