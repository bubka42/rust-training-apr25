use std::arch::x86_64::*;

macro_rules! set_round_key {
    ($i:expr, $round_keys:ident) => {{
        let prev_round_key = $round_keys[$i];
        const RCON: i32 = ROUND_CONSTANTS[$i];
        $round_keys[$i + 1] = unsafe { get_next_round_key::<RCON>(prev_round_key) };
    }};
}

#[repr(C, align(16))]
/// This struct represents the round keys used in AES encryption.
/// It contains an array of 176 bytes, which is the size of the expanded key for AES-128.
pub struct RoundKeys {
    keys: [u8; 176],
}

impl RoundKeys {
    pub fn cast(&self) -> &[__m128i; 11] {
        unsafe { &*(self.keys.as_ptr().cast()) }
    }
}

/// This function expands 128-bit AES keys to round keys.
#[no_mangle]
extern "C" fn expand_key(key: &[u8; 16], rkeys: *mut RoundKeys) {
    // The first round key is the key itself
    let mut round_keys = [unsafe { _mm_setzero_si128() }; 11];
    round_keys[0] = unsafe { _mm_loadu_si128(key.as_ptr() as *const _) };
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

    assert_eq!(size_of::<RoundKeys>(), size_of::<[__m128i; 11]>());
    assert_eq!(align_of::<RoundKeys>(), align_of::<[__m128i; 11]>());
    unsafe {
        core::ptr::write(rkeys.cast(), round_keys);
    }
}

/// This functions gets the next AES round key from the previous round key.
/// # Safety
/// This function is unsafe because it uses SIMD intrinsics that require
/// the target CPU to support them. Ensure that:
/// 1. You are running on a CPU that supports the required SIMD instructions (SSE2, AES-NI).
/// 2. The `key` parameter must be a valid reference to a 16-byte array.
unsafe fn get_next_round_key<const RCON: i32>(prev_round_key: __m128i) -> __m128i {
    // Rotate the last 4 bytes of the previous round key
    let mut next_key = _mm_aeskeygenassist_si128(prev_round_key, RCON);
    next_key = _mm_shuffle_epi32(next_key, 0xFF); // Broadcast the last 4 bytes
    next_key = _mm_xor_si128(next_key, prev_round_key); // XOR with the previous round key
    next_key = _mm_xor_si128(next_key, _mm_slli_si128(prev_round_key, 4));
    next_key = _mm_xor_si128(next_key, _mm_slli_si128(prev_round_key, 8));
    next_key = _mm_xor_si128(next_key, _mm_slli_si128(prev_round_key, 12));
    next_key
}

#[no_mangle]
/// This function encrypts one 128-bit block using AES.
extern "C" fn encrypt1(keys: &RoundKeys, input: &[u8; 16], output: *mut [u8; 16]) {
    // Load the input block
    let mut state = unsafe { _mm_loadu_si128(input.as_ptr() as *const _) };
    // Add the first round key
    state = unsafe { _mm_xor_si128(state, keys.cast()[0]) };
    // Perform 9 rounds of AES encryption
    for key in keys.cast().iter().skip(1).take(9) {
        state = unsafe { _mm_aesenc_si128(state, *key) };
    }
    // Perform the final round of AES encryption
    state = unsafe { _mm_aesenclast_si128(state, keys.cast()[10]) };
    // Store the result
    unsafe {
        _mm_storeu_si128(output.cast(), state);
    }
}

#[no_mangle]
/// This function decrypts one 128-bit block using AES.
extern "C" fn decrypt1(keys: &RoundKeys, input: &[u8; 16], output: *mut [u8; 16]) {
    // Load the input block
    let mut state = unsafe { _mm_loadu_si128(input.as_ptr() as *const _) };
    // Add the first round key
    state = unsafe { _mm_xor_si128(state, keys.cast()[10]) };
    // Perform 9 rounds of AES decryption
    for key in keys.cast().iter().rev().skip(1).take(9) {
        let rk = unsafe { _mm_aesimc_si128(*key) };
        state = unsafe { _mm_aesdec_si128(state, rk) };
    }
    // Perform the final round of AES decryption
    state = unsafe { _mm_aesdeclast_si128(state, keys.cast()[0]) };
    // Store the result
    unsafe {
        _mm_storeu_si128(output.cast(), state);
    }
}

#[no_mangle]
/// This function encrypts eight 128-bit blocks using AES.
extern "C" fn encrypt8(keys: &RoundKeys, input: &[u8; 128], output: *mut [u8; 128]) {
    for i in 0..8 {
        let block = &input[i * 16..(i + 1) * 16];
        // Load the input block
        let mut state = unsafe { _mm_loadu_si128(block.as_ptr() as *const _) };
        // Add the first round key
        state = unsafe { _mm_xor_si128(state, keys.cast()[0]) };
        // Perform 9 rounds of AES encryption
        for key in keys.cast().iter().skip(1).take(9) {
            state = unsafe { _mm_aesenc_si128(state, *key) };
        }
        // Perform the final round of AES encryption
        state = unsafe { _mm_aesenclast_si128(state, keys.cast()[10]) };
        // Store the result
        let mut encrypted_block = [0u8; 16];
        unsafe {
            _mm_storeu_si128(encrypted_block.as_mut_ptr() as *mut _, state);
        }
        unsafe {
            (*output)[i * 16..(i + 1) * 16].copy_from_slice(&encrypted_block);
        }
    }
}

#[no_mangle]
/// This function decrypts eight 128-bit blocks using AES.
extern "C" fn decrypt8(keys: &RoundKeys, input: &[u8; 128], output: *mut [u8; 128]) {
    for i in 0..8 {
        let block = &input[i * 16..(i + 1) * 16];
        // Load the input block
        let mut state = unsafe { _mm_loadu_si128(block.as_ptr() as *const _) };
        // Add the first round key
        state = unsafe { _mm_xor_si128(state, keys.cast()[10]) };
        // Perform 9 rounds of AES decryption
        for key in keys.cast().iter().rev().skip(1).take(9) {
            let rk = unsafe { _mm_aesimc_si128(*key) };
            state = unsafe { _mm_aesdec_si128(state, rk) };
        }
        // Perform the final round of AES decryption
        state = unsafe { _mm_aesdeclast_si128(state, keys.cast()[0]) };
        // Store the result
        let mut decrypted_block = [0u8; 16];
        unsafe {
            _mm_storeu_si128(decrypted_block.as_mut_ptr() as *mut _, state);
        }
        unsafe {
            (*output)[i * 16..(i + 1) * 16].copy_from_slice(&decrypted_block);
        }
    }
}
