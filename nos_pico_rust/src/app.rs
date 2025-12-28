use chrono::Utc;

use crate::{
    nostr::{
        generate_event::{NostrUnsignEvent, generate_kind1_event, generate_signed_event},
        verify,
    },
    serial::SerialManager,
};

fn get_timestamp() -> i64 {
    let now = Utc::now();
    let timestamp = now.timestamp(); // 秒単位

    return timestamp;
}

///ユーザーが接触するアプリケーション部
pub struct NostrApp {
    pub serial: SerialManager,
    pub ws_tx: tokio::sync::mpsc::Sender<std::string::String>,
    pubkey: String,
    pending_event: Option<NostrUnsignEvent>,
}

impl NostrApp {
    pub fn new(
        serial: SerialManager,
        ws_tx: tokio::sync::mpsc::Sender<std::string::String>,
    ) -> Self {
        Self {
            serial,
            ws_tx,
            pubkey: String::new(),
            pending_event: None,
        }
    }

    pub async fn handle_serial_line(&mut self, line: String) {
        let trimmed_line = line.trim();
        if trimmed_line.starts_with("pubkey___:") {
            let raw_pk = trimmed_line.replace("pubkey___:", "");
            // 公開鍵は必ず64文字（32バイトのHex）
            if raw_pk.len() >= 64 {
                self.pubkey = raw_pk[..64].to_string();
                println!("公開鍵を取得: {}", self.pubkey);
            }
        } else if let Some(event) = self.pending_event.take() {
            println!("署名を受信");

            if trimmed_line.len() >= 128 {
                if verify::verify_nostr_signature(
                    &event.id,     // 送信したID
                    &self.pubkey,  // 自分の公開鍵
                    &trimmed_line, // 返ってきた署名
                ) {
                    let signed_event = generate_signed_event(&event, &trimmed_line);
                    let event_json = match serde_json::to_string(&signed_event) {
                        Ok(event) => event,
                        Err(err) => {
                            println!("JSON化に失敗{}", err);
                            return;
                        }
                    };
                    //WebSocket送信キューに追加
                    if let Err(err) = self.ws_tx.send(event_json).await {
                        println!("WebSocket送信キューへの追加に失敗:{}", err);
                    } else {
                        println!("イベントをリレーサーバーに送信");
                    }
                } else {
                    println!("ERROR:署名の検証に失敗しました");
                    println!("{}", serde_json::to_string(&event).unwrap());
                }
            }
        }
    }

    pub async fn handle_user_input(&mut self, message: String) {
        if self.pubkey.is_empty() {
            println!("ERROR:公開鍵が未取得です。");
            return;
        }

        let event = generate_kind1_event(&message, &self.pubkey, &get_timestamp());

        if SerialManager::send_data(&mut self.serial, event.id.as_bytes())
            .await
            .is_ok()
        {
            self.pending_event = Some(event);
            println!("署名機に送信")
        }
    }
}
