use secp256k1::{Secp256k1, XOnlyPublicKey, schnorr::Signature};
use std::str::FromStr;

pub fn verify_nostr_signature(id_hex: &str, pubkey_hex: &str, sig_hex: &str) -> bool {
    let secp = Secp256k1::verification_only();

    // 1. 公開鍵 (32byte) のパース
    let pubkey = match XOnlyPublicKey::from_str(pubkey_hex) {
        Ok(pk) => pk,
        Err(_) => return false,
    };

    // 2. 署名 (64byte) のパース
    let signature = match Signature::from_str(sig_hex) {
        Ok(sig) => sig,
        Err(_) => return false,
    };

    // 3. メッセージ (ID) のパース
    // 16進数文字列をバイト列(Vec<u8>)に変換
    let id_bytes = match hex::decode(id_hex) {
        Ok(b) => b,
        Err(_) => return false,
    };

    // 4. 検証
    // 第二引数に &id_bytes (スライス) を渡す
    // ※ id_bytes は必ず 32バイトである必要があります
    if id_bytes.len() != 32 {
        return false;
    }

    secp.verify_schnorr(&signature, &id_bytes, &pubkey).is_ok()
}
