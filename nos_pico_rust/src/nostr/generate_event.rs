use hex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{self, Digest, Sha256};

use crate::nostr::NostrSignedEvent;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NostrUnsignEvent {
    pub id: String,
    pub pubkey: String,         // 送信者の公開鍵
    pub created_at: i64,        // Unixタイムスタンプ
    pub kind: u32,              // イベントの種類（0: Metadata, 1: Short Text Note等）
    pub tags: Vec<Vec<String>>, // タグの配列
    pub content: String,        // メッセージ本体
}

///署名前のKind1イベントを生成
pub fn generate_kind1_event(content: &str, pubkey: &str, timestamp: &i64) -> NostrUnsignEvent {
    let tags: Vec<Vec<String>> = vec![];
    let id_source_json = json!([0, &pubkey, &timestamp, 1, &tags, &content]);

    let serialized_data = serde_json::to_string(&id_source_json).unwrap();

    let mut hasher = Sha256::new();
    hasher.update(serialized_data.as_bytes());
    let result = hasher.finalize();

    let id: String = hex::encode(result);

    NostrUnsignEvent {
        id: id,
        pubkey: pubkey.to_string(),
        created_at: *timestamp,
        kind: 1,
        tags,
        content: content.to_string(),
    }
}

///署名済みイベントを生成
pub fn generate_signed_event(event: &NostrUnsignEvent, line: &str) -> NostrSignedEvent {
    NostrSignedEvent {
        id: event.id.to_string(),
        pubkey: event.pubkey.to_string(),
        created_at: event.created_at,
        kind: event.kind,
        tags: event.tags.clone(),
        content: event.content.to_string(),
        sig: line.trim().to_string(),
    }
}
