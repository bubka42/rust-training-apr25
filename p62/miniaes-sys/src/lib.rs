#[repr(C, align(16))]
/// This struct represents the round keys used in AES encryption.
/// It contains an array of 176 bytes, which is the size of the expanded key for AES-128.
pub struct RoundKeys {
    _keys: [u8; 176],
}

#[link(name = "miniaes")]
extern "C" {
    pub fn expand_key(key: &[u8; 16], rkeys: *mut RoundKeys);
    pub fn encrypt1(rkeys: &RoundKeys, input: &[u8; 16], output: &mut [u8; 16]);
    pub fn decrypt1(rkeys: &RoundKeys, input: &[u8; 16], output: &mut [u8; 16]);
    pub fn encrypt8(rkeys: &RoundKeys, input: &[u8; 128], output: &mut [u8; 128]);
    pub fn decrypt8(rkeys: &RoundKeys, input: &[u8; 128], output: &mut [u8; 128]);
}
