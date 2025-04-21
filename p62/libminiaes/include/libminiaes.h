#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * This struct represents the round keys used in AES encryption.
 * It contains an array of 176 bytes, which is the size of the expanded key for AES-128.
 */
typedef struct RoundKeys RoundKeys;

/**
 * This function expands 128-bit AES keys to round keys.
 */
void expand_key(const uint8_t (*key)[16], struct RoundKeys *rkeys);

/**
 * This function encrypts one 128-bit block using AES.
 */
void encrypt1(const struct RoundKeys *keys, const uint8_t (*input)[16], uint8_t (*output)[16]);

/**
 * This function decrypts one 128-bit block using AES.
 */
void decrypt1(const struct RoundKeys *keys, const uint8_t (*input)[16], uint8_t (*output)[16]);

/**
 * This function encrypts eight 128-bit blocks using AES.
 */
void encrypt8(const struct RoundKeys *keys, const uint8_t (*input)[128], uint8_t (*output)[128]);

/**
 * This function decrypts eight 128-bit blocks using AES.
 */
void decrypt8(const struct RoundKeys *keys, const uint8_t (*input)[128], uint8_t (*output)[128]);
