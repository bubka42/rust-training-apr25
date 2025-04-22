use hex_literal::hex;
use mini_aes::AESWrapper;

#[test]
fn test_encrypt1_1() {
    let key = hex!("80000000000000000000000000000000");
    let plaintext = hex!("00000000000000000000000000000000");
    let mut ciphertext = [0u8; 16];
    let expected_ciphertext = hex!("0edd33d3c621e546455bd8ba1418bec8");
    let aeswrapper = AESWrapper::from_key(&key);
    aeswrapper.encrypt1(&plaintext, &mut ciphertext);
    assert_eq!(ciphertext, expected_ciphertext);
}

#[test]
fn test_encrypt1_2() {
    let key = hex!("c0000000000000000000000000000000");
    let plaintext = hex!("00000000000000000000000000000000");
    let mut ciphertext = [0u8; 16];
    let expected_ciphertext = hex!("4bc3f883450c113c64ca42e1112a9e87");
    let aeswrapper = AESWrapper::from_key(&key);
    aeswrapper.encrypt1(&plaintext, &mut ciphertext);
    assert_eq!(ciphertext, expected_ciphertext);
}

#[test]
fn test_decrypt1_1() {
    let key = hex!("80000000000000000000000000000000");
    let ciphertext = hex!("0edd33d3c621e546455bd8ba1418bec8");
    let mut plaintext = [0u8; 16];
    let expected_plaintext = hex!("00000000000000000000000000000000");
    let aeswrapper = AESWrapper::from_key(&key);
    aeswrapper.decrypt1(&ciphertext, &mut plaintext);
    assert_eq!(plaintext, expected_plaintext);
}

#[test]
fn test_decrypt1_2() {
    let key = hex!("c0000000000000000000000000000000");
    let ciphertext = hex!("4bc3f883450c113c64ca42e1112a9e87");
    let mut plaintext = [0u8; 16];
    let expected_plaintext = hex!("00000000000000000000000000000000");
    let aeswrapper = AESWrapper::from_key(&key);
    aeswrapper.decrypt1(&ciphertext, &mut plaintext);
    assert_eq!(plaintext, expected_plaintext);
}

#[test]
fn test_encrypt8() {
    let key = hex!("0edd33d3c621e546455bd8ba1418bec8");
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
    let mut ciphertext8 = [0u8; 128];
    let mut ciphertext1 = [0u8; 16];
    let aeswrapper = AESWrapper::from_key(&key);
    aeswrapper.encrypt8(&plaintext8, &mut ciphertext8);
    aeswrapper.encrypt1(&plaintext1, &mut ciphertext1);
    assert_eq!(ciphertext8[16..32], ciphertext1);
}

#[test]
fn test_decrypt8() {
    let key = hex!("4bc3f883450c113c64ca42e1112a9e87");
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
    let mut plaintext8 = [0u8; 128];
    let mut plaintext1 = [0u8; 16];
    let aeswrapper = AESWrapper::from_key(&key);
    aeswrapper.encrypt8(&ciphertext8, &mut plaintext8);
    aeswrapper.encrypt1(&ciphertext1, &mut plaintext1);
    assert_eq!(plaintext8[64..80], plaintext1);
}
