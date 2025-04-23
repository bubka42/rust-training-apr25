use aes_siv::{
    aead::{Aead, AeadMutInPlace, KeyInit, OsRng},
    AeadCore, Aes128SivAead, Key, Nonce, Tag,
};
use std::io::{Read, Write};

fn aes128siv_encrypt(key: &[u8; 32], nonce: Nonce, plaintext: &[u8]) -> Vec<u8> {
    let key = Key::<Aes128SivAead>::from_slice(key);
    let cipher = Aes128SivAead::new(key);
    cipher
        .encrypt(&nonce, plaintext)
        .expect("encryption failure!")
}

fn aes128siv_decrypt(key: &[u8; 32], nonce: Nonce, ciphertext: &[u8]) -> Vec<u8> {
    let key = Key::<Aes128SivAead>::from_slice(key);
    let cipher = Aes128SivAead::new(key);
    cipher
        .decrypt(&nonce, ciphertext)
        .expect("decryption failure!")
}

fn aes128siv_encrypt_ipd(key: &[u8; 32], nonce: Nonce, ad: &[u8], buffer: &mut [u8]) -> Tag {
    let key = Key::<Aes128SivAead>::from_slice(key);
    let mut cipher = Aes128SivAead::new(key);
    cipher
        .encrypt_in_place_detached(&nonce, ad, buffer)
        .expect("encryption failure!")
}

fn aes128siv_decrypt_ipd(key: &[u8; 32], nonce: Nonce, ad: &[u8], buffer: &mut [u8], tag: Tag) {
    let key = Key::<Aes128SivAead>::from_slice(key);
    let mut cipher = Aes128SivAead::new(key);
    cipher
        .decrypt_in_place_detached(&nonce, ad, buffer, &tag)
        .expect("encryption failure!");
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
    let mut key = [0u8; 32];
    base16ct::mixed::decode(&args[4], &mut key).unwrap();
    let nonce = Aes128SivAead::generate_nonce(&mut OsRng);
    match args[1].as_str() {
        "enc" => {
            let plaintext = std::fs::read(input_path).expect("Failed to read input file");
            let ciphertext = aes128siv_encrypt(&key, nonce, &plaintext);
            // Open output file for writing
            let output_file =
                std::fs::File::create(output_path).expect("Failed to create output file");
            // Write the ciphertext and nonce to the output file
            let mut output = std::io::BufWriter::new(output_file);
            output.write_all(&nonce).expect("Failed to write nonce");
            output
                .write_all(&ciphertext)
                .expect("Failed to write ciphertext");
        }
        "encipd" => {
            // Open input file for reading
            let mut input_file =
                std::fs::File::open(input_path).expect("Failed to open input file");
            // Read the plaintext from the input file
            let mut buffer = Vec::new();
            input_file
                .read_to_end(&mut buffer)
                .expect("Failed to read plaintext from input file");
            // Encrypt the plaintext in place with associated data
            let tag = aes128siv_encrypt_ipd(&key, nonce, &[], &mut buffer);
            // Open output file for writing
            let output_file =
                std::fs::File::create(output_path).expect("Failed to create output file");
            // Write the ciphertext and nonce to the output file
            let mut output = std::io::BufWriter::new(output_file);
            output.write_all(&nonce).expect("Failed to write nonce");
            output.write_all(&tag).expect("Failed to write tag");
            output
                .write_all(&buffer)
                .expect("Failed to write ciphertext");
        }
        "dec" => {
            // Open input file for reading
            let input_file = std::fs::File::open(input_path).expect("Failed to open input file");
            // Read the nonce from the input file
            let mut nonce = Nonce::default();
            let mut input = std::io::BufReader::new(input_file);
            input
                .read_exact(&mut nonce)
                .expect("Failed to read nonce from input file");
            // Read the ciphertext from the input file
            let mut ciphertext = Vec::new();
            input
                .read_to_end(&mut ciphertext)
                .expect("Failed to read ciphertext from input file");
            // Decrypt the ciphertext
            let plaintext = aes128siv_decrypt(&key, nonce, &ciphertext);
            std::fs::write(output_path, plaintext).expect("Failed to write output file");
        }
        "decipd" => {
            // Open input file for reading
            let input_file = std::fs::File::open(input_path).expect("Failed to open input file");
            // Read the nonce from the input file
            let mut nonce = Nonce::default();
            let mut input = std::io::BufReader::new(input_file);
            input
                .read_exact(&mut nonce)
                .expect("Failed to read nonce from input file");
            // Read the tag from the input file
            let mut tag = Tag::default();
            input
                .read_exact(&mut tag)
                .expect("Failed to read tag from input file");
            // Read the ciphertext from the input file
            let mut buffer = Vec::new();
            input
                .read_to_end(&mut buffer)
                .expect("Failed to read ciphertext from input file");
            // Decrypt the ciphertext in place with associated data
            aes128siv_decrypt_ipd(&key, nonce, &[], &mut buffer, tag);
            std::fs::write(output_path, buffer).expect("Failed to write output file");
        }
        _ => {
            eprintln!("Unsupported operation: {}", args[1]);
            std::process::exit(1);
        }
    }
}
