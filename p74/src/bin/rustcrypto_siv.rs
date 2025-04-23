use aes_siv::{
    aead::{Aead, KeyInit, OsRng},
    AeadCore, Aes128SivAead, Key, Nonce,
};

fn aes128siv_encrypt(key: &[u8; 16], nonce: Nonce, plaintext: &[u8]) -> Vec<u8> {
    let key = Key::<Aes128SivAead>::from_slice(key);
    let cipher = Aes128SivAead::new(key);
    cipher
        .encrypt(&nonce, plaintext)
        .expect("encryption failure!")
}

fn aes128siv_decrypt(key: &[u8; 16], nonce: Nonce, ciphertext: &[u8]) -> Vec<u8> {
    let key = Key::<Aes128SivAead>::from_slice(key);
    let cipher = Aes128SivAead::new(key);
    cipher
        .decrypt(&nonce, ciphertext)
        .expect("decryption failure!")
}

fn main() {
    // Get the input string from command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 5 {
        eprintln!("Usage: {} <input_string>", args[0]);
        std::process::exit(1);
    }
    let input_path = &args[2];
    let output_path = &args[3];
    let mut key = [0u8; 16];
    base16ct::mixed::decode(&args[4], &mut key).unwrap();
    let nonce = Aes128SivAead::generate_nonce(&mut OsRng);
    match args[1].as_str() {
        "enc" => {
            let plaintext = std::fs::read(input_path).expect("Failed to read input file");
            let ciphertext = aes128siv_encrypt(&key, nonce, &plaintext);
            std::fs::write(output_path, ciphertext).expect("Failed to write output file");
        }
        "dec" => {
            let ciphertext = std::fs::read(input_path).expect("Failed to read input file");
            let plaintext = aes128siv_decrypt(&key, nonce, &ciphertext);
            std::fs::write(output_path, plaintext).expect("Failed to write output file");
        }
        _ => {
            eprintln!("Unsupported operation: {}", args[1]);
            std::process::exit(1);
        }
    }
}
