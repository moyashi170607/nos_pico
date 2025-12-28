/* これらを定義することで機能を有効化する */
#define ENABLE_MODULE_EXTRAKEYS 1
#define ENABLE_MODULE_SCHNORRSIG 1

/* さらに、x86等の特殊命令を使わないよう指定 (マイコン向け) */
#define ECMULT_WINDOW_SIZE 15
#define USE_EXTERNAL_DEFAULT_CALLBACKS 1

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "config.hpp"
#include "hardware/structs/rosc.h"
#include "hardware/uart.h"
#include "nostr_sign.hpp"
#include "pico/stdlib.h"

#define BUILTIN_LED 25

#define BUFFER_MAX 70

unsigned char msg_buffer[BUFFER_MAX];
int buffer_index = 0;

unsigned char copy_buffer[BUFFER_MAX];

void copy_trim_msg(unsigned char msg[], unsigned char copy_msg[], int length) {
    for (int i = 0; i < length; i++) {
        unsigned char temp_char = msg[i];
        if (temp_char == '\n' || temp_char == '\r') {
            break;
        } else {
            copy_msg[i] = msg[i];
        }
    }
    copy_msg[length] = '\0';
}

int main() {
    stdio_init_all();

    gpio_init(BUILTIN_LED);
    gpio_set_dir(BUILTIN_LED, GPIO_OUT);

    sk_hex_to_byte(seckey_hex);

    while (true) {
        // 1文字取得
        int c = getchar_timeout_us(0);

        if (c == PICO_ERROR_TIMEOUT) {
            // 入力がない場合は待機
            continue;
        }

        // 改行文字（CRまたはLF）を判定
        if (c == '\r' || c == '\n') {
            msg_buffer[buffer_index] = '\0';  // 終端文字を付与

            if (buffer_index > 0) {
                if (strncmp((char*)msg_buffer, "get_pubkey", 10) == 0) {
                    unsigned char pubkey[PUB_KEY_LENGTH];
                    get_public_key(pubkey);

                    // 結果の表示
                    printf("pubkey___:");
                    for (int i = 0; i < PUB_KEY_LENGTH; i++) {
                        printf("%02x", pubkey[i]);
                    }
                    printf("\n");
                } else {
                    copy_trim_msg(msg_buffer, copy_buffer, buffer_index);
                    // unsigned char id_bin[BUFFER_MAX - 1];
                    sign(copy_buffer);
                }
                buffer_index = 0;  // 次の入力のためにリセット
            }

        } else {
            // バッファサイズ内で文字を格納
            if (buffer_index < sizeof(msg_buffer) - 1) {
                msg_buffer[buffer_index++] = (char)c;
            }
        }
    }

    return EXIT_SUCCESS;
}