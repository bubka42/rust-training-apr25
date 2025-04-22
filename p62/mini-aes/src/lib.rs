use miniaes_sys as sys;
use std::mem::MaybeUninit;

pub struct AESWrapper {
    keys: sys::RoundKeys,
}

impl Default for AESWrapper {
    fn default() -> Self {
        Self::new()
    }
}

impl AESWrapper {
    pub fn new() -> Self {
        Self {
            keys: unsafe { MaybeUninit::zeroed().assume_init() },
        }
    }

    pub fn from_key(key: &[u8; 16]) -> Self {
        let mut keys = Self::new();
        unsafe {
            sys::expand_key(key, &mut keys.keys);
        }
        keys
    }

    pub fn encrypt1(&self, input: &[u8; 16], output: &mut [u8; 16]) {
        unsafe {
            sys::encrypt1(&self.keys, input, output);
        }
    }
    pub fn decrypt1(&self, input: &[u8; 16], output: &mut [u8; 16]) {
        unsafe {
            sys::decrypt1(&self.keys, input, output);
        }
    }
    pub fn encrypt8(&self, input: &[u8; 128], output: &mut [u8; 128]) {
        unsafe {
            sys::encrypt8(&self.keys, input, output);
        }
    }
    pub fn decrypt8(&self, input: &[u8; 128], output: &mut [u8; 128]) {
        unsafe {
            sys::decrypt8(&self.keys, input, output);
        }
    }
}
