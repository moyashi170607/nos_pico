use serde::{Deserialize, Serialize};

pub mod generate_event;
pub mod verify;
pub mod web_socket;
pub mod ws_send_tx;

///署名済みkind1イベント
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NostrSignedEvent {
    pub id: String,
    pub pubkey: String,         // 送信者の公開鍵
    pub created_at: i64,        // Unixタイムスタンプ
    pub kind: u32,              // イベントの種類（0: Metadata, 1: Short Text Note等）
    pub tags: Vec<Vec<String>>, // タグの配列
    pub content: String,        // メッセージ本体
    pub sig: String,
}
