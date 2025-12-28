use futures::stream::SplitSink;
use tokio::{
    net::TcpStream,
    sync::mpsc::{self, Sender},
};
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream,
    tungstenite::{self, Message},
};

use futures::SinkExt;

pub fn start_ws_send_tx(
    ws_writers: Vec<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>,
) -> Sender<String> {
    //送受信機を作成
    //txにデータを入れることで送信できる。rxはtxに入れらえたデータを取り出す
    let (ws_send_tx, mut ws_send_rx) = mpsc::channel::<String>(100);

    //txに入ったデータをリレーサーバーに送信
    //tokioによって非同期で別タスクとして処理される
    tokio::spawn(async move {
        let mut writers = ws_writers;
        while let Some(json_msg) = ws_send_rx.recv().await {
            // format! で作った String を用意
            let req_str = format!("[\"EVENT\",{}]", json_msg);

            println!("{}", req_str);

            // String を Utf8Bytes に変換して Message::Text を作成
            let msg = tungstenite::protocol::Message::Text(req_str.into()); // .into() で変換可能

            //順番にリレーサーバーに送信
            for writer in writers.iter_mut() {
                // ここで複製が必要な場合は msg.clone()
                let _ = writer.send(msg.clone()).await;
                println!("リレー送信");
            }
        }
    });

    ws_send_tx
}
