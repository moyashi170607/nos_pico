use futures::{
    StreamExt,
    stream::{SplitSink, SplitStream},
};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite::Message};

///WebSocketを開く
pub async fn open_ws(
    relays: Vec<String>,
) -> (
    Vec<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>,
    Vec<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
) {
    let mut ws_stream_vec: Vec<WebSocketStream<MaybeTlsStream<TcpStream>>> = vec![];

    // WebSocket接続を開始
    for i in relays {
        let (ws_stream, _) = connect_async(i).await.expect("Failed to connect");
        ws_stream_vec.push(ws_stream);
    }

    println!("Connect");

    let mut ws_writers: Vec<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>> =
        vec![];
    let mut ws_readers: Vec<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>> = vec![];

    //書き込みと読み込みに分ける
    for i in ws_stream_vec {
        let (writer, reader) = i.split();
        ws_writers.push(writer);
        ws_readers.push(reader);
    }

    //タプル型で返す
    (ws_writers, ws_readers)
}
