#include "nostr_sign.hpp"

#include <secp256k1_schnorrsig.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "pico/rand.h"
#include "pico/stdlib.h"

unsigned char seckey[] = {};

// 16進数文字を数値に変換する補助関数
uint8_t hex_to_uint8(char c) {
    if (c >= '0' && c <= '9') return c - '0';
    if (c >= 'a' && c <= 'f') return c - 'a' + 10;
    if (c >= 'A' && c <= 'F') return c - 'A' + 10;
    return 0;
}

// 16進数文字列(64文字)をバイナリ(32バイト)に変換
void hex_to_bytes(unsigned const char* hex, uint8_t* bytes) {
    for (int i = 0; i < 32; i++) {
        bytes[i] =
            (hex_to_uint8(hex[i * 2]) << 4) | hex_to_uint8(hex[i * 2 + 1]);
    }
}

// 公開鍵の算出
int get_public_key(unsigned char* result_chars) {
    secp256k1_context* ctx = secp256k1_context_create(SECP256K1_CONTEXT_SIGN);

    // 4. キーペアと公開鍵の作成
    secp256k1_keypair keypair;
    secp256k1_xonly_pubkey pubkey;

    if (!secp256k1_keypair_create(ctx, &keypair, seckey)) return EXIT_FAILURE;
    if (!secp256k1_keypair_xonly_pub(ctx, &pubkey, NULL, &keypair))
        return EXIT_FAILURE;

    if (!secp256k1_xonly_pubkey_serialize(ctx, result_chars, &pubkey)) {
        secp256k1_context_destroy(ctx);
        return EXIT_FAILURE;
    }

    return EXIT_SUCCESS;
}

// 秘密鍵をhexからUint8Arrayに変換し、グローバル変数に格納
int sk_hex_to_byte(const char* hex) {
    for (int i = 0; i < 32; i++) {
        seckey[i] =
            (hex_to_uint8(hex[i * 2]) << 4) | hex_to_uint8(hex[i * 2 + 1]);
    }

    return EXIT_SUCCESS;
}

int sign(unsigned char* msg_hex) {
    unsigned char msg_hash[32];
    hex_to_bytes(msg_hex, msg_hash);
    // コンテキストの作成
    secp256k1_context* ctx = secp256k1_context_create(SECP256K1_CONTEXT_SIGN);

    // 4. キーペアと公開鍵の作成
    secp256k1_keypair keypair;
    secp256k1_xonly_pubkey pubkey;

    if (!secp256k1_keypair_create(ctx, &keypair, seckey)) return EXIT_FAILURE;
    if (!secp256k1_keypair_xonly_pub(ctx, &pubkey, NULL, &keypair))
        return EXIT_FAILURE;

    // 5. 署名の実行 (Schnorr署名)
    unsigned char signature[64];
    // aux_rand32は補助的な乱数（NULLでも可ですが、サイドチャネル攻撃対策として推奨されます）
    unsigned char aux_rand32[32] = {};
    for (int i = 0; i < 32; i += 4) {
        uint32_t r = get_rand_32();
        memcpy(&aux_rand32[i], &r, 4);
    }

    if (!secp256k1_schnorrsig_sign32(ctx, signature, msg_hash, &keypair,
                                     aux_rand32)) {
        return EXIT_FAILURE;
    }

    // 6. 結果の表示 (16進数)
    for (int i = 0; i < 64; i++) printf("%02x", signature[i]);
    printf("\n");

    // リソースの解放
    secp256k1_context_destroy(ctx);
    return EXIT_SUCCESS;
}
