#ifndef NOSTR_SIGN_HPP
#define NOSTR_SIGN_HPP

#include <cstdint>

#define PUB_KEY_LENGTH 64

// 16進数文字を数値に変換する補助関数
uint8_t hex_to_uint8(char c);

// 16進数文字列(64文字)をバイナリ(32バイト)に変換
void hex_to_bytes(unsigned const char* hex, uint8_t* bytes);

// シュノア署名
int sign(unsigned char* msg_hex);

int get_public_key(unsigned char* result_chars);

int sk_hex_to_byte(const char* hex);

#endif  // SIGN_HPP
