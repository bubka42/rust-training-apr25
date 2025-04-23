use aes::{
    cipher::{KeyIvInit, StreamCipher},
    Aes128,
};
use cmac::{digest::CtOutput, Cmac, Mac};
use std::io::{Read, Write};

type Aes128Ctr64LE = ctr::Ctr64LE<Aes128>;

const KEY_SIZE: usize = 16; // AES-128 key size in bytes
const IV_SIZE: usize = 16; // AES-128 IV size in bytes
const TAG_SIZE: usize = 16; // CMAC tag size in bytes

fn cmac_tag(key: &[u8; KEY_SIZE], data: &[u8]) -> CtOutput<Cmac<Aes128>> {
    let mut cmac = Cmac::<Aes128>::new_from_slice(key).expect("Invalid key length");
    cmac.update(data);
    cmac.finalize()
}

fn ctr_encrypt(key: &[u8; KEY_SIZE], iv: &[u8; IV_SIZE], data: &mut [u8]) {
    let mut cipher = Aes128Ctr64LE::new_from_slices(key, iv).expect("Invalid key or IV length");
    let mut buffer = data.to_vec();
    cipher.apply_keystream(&mut buffer);
}

fn main() {
    // Get the input string from command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 5 {
        eprintln!("Usage: {} <input_string>", args[0]);
        std::process::exit(1);
    }
    let input_path = &args[2];
    // Open input file for reading
    let mut input_file = std::fs::File::open(input_path).expect("Failed to open input file");
    let output_path = &args[3];
    // Open output file for writing
    let output_file = std::fs::File::create(output_path).expect("Failed to create output file");
    let mut output = std::io::BufWriter::new(output_file);
    let mut key = [0u8; KEY_SIZE];
    base16ct::mixed::decode(&args[4], &mut key).unwrap();
    let mut buffer = Vec::new();
    match args[1].as_str() {
        "enc" => {
            let mut iv = [0u8; IV_SIZE]; // 16 bytes IV for AES-128 CTR
            getrandom::fill(&mut iv).expect("Failed to generate IV");
            input_file
                .read_to_end(&mut buffer)
                .expect("Failed to read plaintext");
            ctr_encrypt(&key, &iv, &mut buffer);
            let tag = cmac_tag(&key, &buffer).into_bytes();
            output.write_all(&iv).expect("Failed to write IV");
            output.write_all(&tag).expect("Failed to write tag");
            output
                .write_all(&buffer)
                .expect("Failed to write ciphertext");
        }
        "dec" => {
            let mut iv = [0u8; IV_SIZE]; // 16 bytes IV for AES-128 CTR
            let mut tag = [0u8; TAG_SIZE]; // 16 bytes tag for CMAC
            input_file.read_exact(&mut iv).expect("Failed to read IV");
            input_file.read_exact(&mut tag).expect("Failed to read tag");
            input_file
                .read_to_end(&mut buffer)
                .expect("Failed to read ciphertext");
            let tag_verify = cmac_tag(&key, &buffer);
            if tag.to_vec() != tag_verify.into_bytes().to_vec() {
                eprintln!("Tag verification failed. Data may be tampered.");
                std::process::exit(1);
            }
            ctr_encrypt(&key, &iv, &mut buffer);
            // Write the plaintext to the output file
            output
                .write_all(&buffer)
                .expect("Failed to write plaintext");
        }
        _ => {
            eprintln!("Invalid operation. Use 'enc' for encryption or 'dec' for decryption.");
        }
    }
}
