use std::arch::x86_64::*;

macro_rules! set_round_key {
    ($i:expr, $round_keys:ident) => {{
        let prev_round_key = $round_keys[$i];
        const RCON: i32 = ROUND_CONSTANTS[$i];
        $round_keys[$i + 1] = get_next_round_key::<RCON>(prev_round_key);
    }};
}

/// This function expands 128-bit AES keys to round keys.
/// # Safety
/// This function is unsafe because it uses SIMD intrinsics that require
/// the target CPU to support them. Ensure that:
/// 1. You are running on a CPU that supports the required SIMD instructions (SSE2, AES-NI).
/// 2. The `key` parameter must be a valid reference to a 16-byte array.
pub unsafe fn expand_key(key: &[u8; 16]) -> [__m128i; 11] {
    // The first round key is the key itself
    let mut round_keys = [_mm_setzero_si128(); 11];
    round_keys[0] = _mm_loadu_si128(key.as_ptr() as *const _);
    // The rest of the round keys are generated from the previous round key
    // Precomputed round constants for AES key schedule
    const ROUND_CONSTANTS: [i32; 10] = [1, 2, 4, 8, 16, 32, 64, 128, 27, 54];

    set_round_key!(0, round_keys);
    set_round_key!(1, round_keys);
    set_round_key!(2, round_keys);
    set_round_key!(3, round_keys);
    set_round_key!(4, round_keys);
    set_round_key!(5, round_keys);
    set_round_key!(6, round_keys);
    set_round_key!(7, round_keys);
    set_round_key!(8, round_keys);
    set_round_key!(9, round_keys);

    round_keys
}

/// This functions gets the next AES round key from the previous round key.
/// # Safety
/// This function is unsafe because it uses SIMD intrinsics that require
/// the target CPU to support them. Ensure that:
/// 1. You are running on a CPU that supports the required SIMD instructions (SSE2, AES-NI).
/// 2. The `key` parameter must be a valid reference to a 16-byte array.
pub unsafe fn get_next_round_key<const RCON: i32>(prev_round_key: __m128i) -> __m128i {
    // Rotate the last 4 bytes of the previous round key
    let mut next_key = _mm_aeskeygenassist_si128(prev_round_key, RCON);
    next_key = _mm_shuffle_epi32(next_key, 0xFF); // Broadcast the last 4 bytes
    next_key = _mm_xor_si128(next_key, prev_round_key); // XOR with the previous round key
    next_key = _mm_xor_si128(next_key, _mm_slli_si128(prev_round_key, 4));
    next_key = _mm_xor_si128(next_key, _mm_slli_si128(prev_round_key, 8));
    next_key = _mm_xor_si128(next_key, _mm_slli_si128(prev_round_key, 12));
    next_key
}

/// This function encrypts one 128-bit block using AES.
/// # Safety
/// This function is unsafe because it uses SIMD intrinsics that require
/// the target CPU to support them. Ensure that:
/// 1. You are running on a CPU that supports the required SIMD instructions (SSE2, AES-NI).
/// 2. The `block` parameter must be a valid reference to a 16-byte array.
/// 3. The `keys` parameter must be a valid reference to an array of 11 128-bit keys.
/// 4. The `keys` array must be generated using the `expand_key` function.
pub unsafe fn encrypt1(keys: &[__m128i; 11], block: &[u8; 16]) -> [u8; 16] {
    // Load the input block
    let mut state = _mm_loadu_si128(block.as_ptr() as *const _);
    // Add the first round key
    state = _mm_xor_si128(state, keys[0]);
    // Perform 9 rounds of AES encryption
    for key in keys.iter().skip(1).take(9) {
        state = _mm_aesenc_si128(state, *key);
    }
    // Perform the final round of AES encryption
    state = _mm_aesenclast_si128(state, keys[10]);
    // Store the result
    let mut output = [0u8; 16];
    _mm_storeu_si128(output.as_mut_ptr() as *mut _, state);
    output
}

/// This function decrypts one 128-bit block using AES.
/// # Safety
/// This function is unsafe because it uses SIMD intrinsics that require
/// the target CPU to support them. Ensure that:
/// 1. You are running on a CPU that supports the required SIMD instructions (SSE2, AES-NI).
/// 2. The `block` parameter must be a valid reference to a 16-byte array.
/// 3. The `keys` parameter must be a valid reference to an array of 11 128-bit keys.
/// 4. The `keys` array must be generated using the `expand_key` function.
pub unsafe fn decrypt1(keys: &[__m128i; 11], block: &[u8; 16]) -> [u8; 16] {
    // Load the input block
    let mut state = _mm_loadu_si128(block.as_ptr() as *const _);
    // Add the first round key
    state = _mm_xor_si128(state, keys[10]);
    // Perform 9 rounds of AES decryption
    for key in keys.iter().rev().skip(1).take(9) {
        let rk = _mm_aesimc_si128(*key);
        state = _mm_aesdec_si128(state, rk);
    }
    // Perform the final round of AES decryption
    state = _mm_aesdeclast_si128(state, keys[0]);
    // Store the result
    let mut output = [0u8; 16];
    _mm_storeu_si128(output.as_mut_ptr() as *mut _, state);
    output
}

/// This function encrypts eight 128-bit blocks using AES.
/// # Safety
/// This function is unsafe because it uses SIMD intrinsics that require
/// the target CPU to support them. Ensure that:
/// 1. You are running on a CPU that supports the required SIMD instructions (SSE2, AES-NI).
/// 2. The `blocks` parameter must be a valid reference to an array of 128 bytes.
/// 3. The `keys` parameter must be a valid reference to an array of 11 128-bit keys.
/// 4. The `keys` array must be generated using the `expand_key` function.
pub unsafe fn encrypt8(keys: &[__m128i; 11], blocks: &[u8; 128]) -> [u8; 128] {
    let mut output = [0u8; 128];
    for i in 0..8 {
        let block = &blocks[i * 16..(i + 1) * 16];
        // Load the input block
        let mut state = _mm_loadu_si128(block.as_ptr() as *const _);
        // Add the first round key
        state = _mm_xor_si128(state, keys[0]);
        // Perform 9 rounds of AES encryption
        for key in keys.iter().skip(1).take(9) {
            state = _mm_aesenc_si128(state, *key);
        }
        // Perform the final round of AES encryption
        state = _mm_aesenclast_si128(state, keys[10]);
        // Store the result
        let mut encrypted_block = [0u8; 16];
        _mm_storeu_si128(encrypted_block.as_mut_ptr() as *mut _, state);
        output[i * 16..(i + 1) * 16].copy_from_slice(&encrypted_block);
    }
    output
}

/// This function decrypts eight 128-bit blocks using AES.
/// # Safety
/// This function is unsafe because it uses SIMD intrinsics that require
/// the target CPU to support them. Ensure that:
/// 1. You are running on a CPU that supports the required SIMD instructions (SSE2, AES-NI).
/// 2. The `blocks` parameter must be a valid reference to a 16-byte array.
/// 3. The `keys` parameter must be a valid reference to an array of 11 128-bit keys.
/// 4. The `keys` array must be generated using the `expand_key` function.
pub unsafe fn decrypt8(keys: &[__m128i; 11], blocks: &[u8; 128]) -> [u8; 128] {
    let mut output = [0u8; 128];
    for i in 0..8 {
        let block = &blocks[i * 16..(i + 1) * 16];
        // Load the input block
        let mut state = _mm_loadu_si128(block.as_ptr() as *const _);
        // Add the first round key
        state = _mm_xor_si128(state, keys[10]);
        // Perform 9 rounds of AES decryption
        for key in keys.iter().rev().skip(1).take(9) {
            let rk = _mm_aesimc_si128(*key);
            state = _mm_aesdec_si128(state, rk);
        }
        // Perform the final round of AES decryption
        state = _mm_aesdeclast_si128(state, keys[0]);
        // Store the result
        let mut decrypted_block = [0u8; 16];
        _mm_storeu_si128(decrypted_block.as_mut_ptr() as *mut _, state);
        output[i * 16..(i + 1) * 16].copy_from_slice(&decrypted_block);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn test_expand_key() {
        let key: [u8; 16] = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F,
        ];
        unsafe {
            let round_keys = expand_key(&key);
            assert_eq!(round_keys.len(), 11);
        }
    }
    #[test]
    fn test_expand_key_2() {
        let key: [u8; 16] = [
            0x54, 0x68, 0x61, 0x74, 0x73, 0x20, 0x6D, 0x79, 0x20, 0x4B, 0x75, 0x6E, 0x67, 0x20,
            0x46, 0x75,
        ];
        let final_key: [u8; 16] = [
            0x28, 0xFD, 0xDE, 0xF8, 0x6D, 0xA4, 0x24, 0x4A, 0xCC, 0xC0, 0xA4, 0xFE, 0x3B, 0x31,
            0x6F, 0x26,
        ];
        let mut final_key_2 = [0u8; 16];
        unsafe {
            let round_keys = expand_key(&key);
            _mm_storeu_si128(final_key_2.as_mut_ptr() as *mut _, round_keys[10]);
        }
        assert_eq!(final_key_2, final_key);
    }

    #[test]
    fn test_encrypt1_1() {
        let key = hex!("80000000000000000000000000000000");
        let plaintext = hex!("00000000000000000000000000000000");
        let expected_ciphertext = hex!("0edd33d3c621e546455bd8ba1418bec8");
        unsafe {
            let round_keys = expand_key(&key);
            let ciphertext = encrypt1(&round_keys, &plaintext);
            assert_eq!(ciphertext, expected_ciphertext);
        }
    }

    #[test]
    fn test_encrypt1_2() {
        let key = hex!("c0000000000000000000000000000000");
        let plaintext = hex!("00000000000000000000000000000000");
        let expected_ciphertext = hex!("4bc3f883450c113c64ca42e1112a9e87");
        unsafe {
            let round_keys = expand_key(&key);
            let ciphertext = encrypt1(&round_keys, &plaintext);
            assert_eq!(ciphertext, expected_ciphertext);
        }
    }

    #[test]
    fn test_decrypt1_1() {
        let key = hex!("80000000000000000000000000000000");
        let ciphertext = hex!("0edd33d3c621e546455bd8ba1418bec8");
        let expected_plaintext = hex!("00000000000000000000000000000000");
        unsafe {
            let round_keys = expand_key(&key);
            let plaintext = decrypt1(&round_keys, &ciphertext);
            assert_eq!(plaintext, expected_plaintext);
        }
    }

    #[test]
    fn test_decrypt1_2() {
        let key = hex!("c0000000000000000000000000000000");
        let ciphertext = hex!("4bc3f883450c113c64ca42e1112a9e87");
        let expected_plaintext = hex!("00000000000000000000000000000000");
        unsafe {
            let round_keys = expand_key(&key);
            let plaintext = decrypt1(&round_keys, &ciphertext);
            assert_eq!(plaintext, expected_plaintext);
        }
    }

    #[test]
    fn test_encrypt8() {
        let key = hex!("0edd33d3c621e546455bd8ba1418bec8");
        let round_keys = unsafe { expand_key(&key) };
        let plaintext8 = hex!(
            "00000000000000000000000000000000"
            "01010101010101010101010101010101"
            "02020202020202020202020202020202"
            "03030303030303030303030303030303"
            "04040404040404040404040404040404"
            "05050505050505050505050505050505"
            "06060606060606060606060606060606"
            "07070707070707070707070707070707"
        );
        let plaintext1 = hex!("01010101010101010101010101010101");
        let ciphertext8 = unsafe { encrypt8(&round_keys, &plaintext8) };
        let ciphertext1 = unsafe { encrypt1(&round_keys, &plaintext1) };
        assert_eq!(ciphertext8[16..32], ciphertext1);
    }

    #[test]
    fn test_decrypt8() {
        let key = hex!("4bc3f883450c113c64ca42e1112a9e87");
        let round_keys = unsafe { expand_key(&key) };
        let ciphertext8 = hex!(
            "00000000000000000000000000000000"
            "01010101010101010101010101010101"
            "02020202020202020202020202020202"
            "03030303030303030303030303030303"
            "04040404040404040404040404040404"
            "05050505050505050505050505050505"
            "06060606060606060606060606060606"
            "07070707070707070707070707070707"
        );
        let ciphertext1 = hex!("04040404040404040404040404040404");
        let plaintext8 = unsafe { encrypt8(&round_keys, &ciphertext8) };
        let plaintext1 = unsafe { encrypt1(&round_keys, &ciphertext1) };
        assert_eq!(plaintext8[64..80], plaintext1);
    }
}
