use tokio::io::{self, AsyncWriteExt};
use tokio_serial::{SerialPort, SerialPortBuilderExt, SerialStream};
use tokio_util::{bytes::BytesMut, codec::Decoder};

pub struct LineCodec;

//シリアル通信を読む
impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        //改行コードを探す
        let newline = src.as_ref().iter().position(|b| *b == b'\n');
        //改行コードまでを切り取る
        if let Some(n) = newline {
            let line = src.split_to(n + 1);
            return match str::from_utf8(line.as_ref()) {
                //文字列化
                Ok(s) => Ok(Some(s.to_string())),
                //エラー
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Invalid String")),
            };
        }
        //まだ改行コードまで来てないので待機
        Ok(None)
    }
}

//シリアル通信の管理
pub struct SerialManager {
    //port: SerialStream,
    pub reader: tokio_util::codec::Framed<SerialStream, LineCodec>,
}

impl SerialManager {
    ///指定したポートとシリアル通信を開く
    pub async fn open_port(
        port_name: &str,
        baud_rate: u32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // ? を使って Result から SerialStream を取り出す
        let mut port: SerialStream = tokio_serial::new(port_name, baud_rate).open_native_async()?;

        //良く分からないけど必要らしいね
        port.write_data_terminal_ready(true)?;

        //ポートを利用しやすい形に変
        let reader: tokio_util::codec::Framed<SerialStream, LineCodec> = LineCodec.framed(port);

        // 取り出した port を構造体にセットして返す
        Ok(SerialManager { reader })
    }

    ///データの送信
    pub async fn send_data(&mut self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        self.reader.get_mut().write_all(data).await?; // 送信失敗ならここで中断して戻る
        self.reader.get_mut().write_all(b"\n").await?; // 改行コード（\n）を送信
        Ok(()) // 成功なら「空の値 ()」を Ok で包んで返す
    }
}
