use base16ct::lower::encode_string;
use sha3::{Digest, Sha3_224, Sha3_256, Sha3_384, Sha3_512, Shake128, Shake256};

fn sha_224(input_string: &str) -> String {
    let mut hasher = Sha3_224::new();
    hasher.update(input_string.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn sha_256(input_string: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(input_string.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn sha_384(input_string: &str) -> String {
    let mut hasher = Sha3_384::new();
    hasher.update(input_string.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn sha_512(input_string: &str) -> String {
    let mut hasher = Sha3_512::new();
    hasher.update(input_string.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn shake_128(input_string: &str, output_size: usize) -> String {
    use sha3::digest::{ExtendableOutput, Update};
    let mut hasher = Shake128::default();
    hasher.update(input_string.as_bytes());
    let mut output = vec![0u8; output_size];
    hasher.finalize_xof_into(&mut output);
    encode_string(&output)
}

fn shake_256(input_string: &str, output_size: usize) -> String {
    use sha3::digest::{ExtendableOutput, Update};
    let mut hasher = Shake256::default();
    hasher.update(input_string.as_bytes());
    let mut output = vec![0u8; output_size];
    hasher.finalize_xof_into(&mut output);
    encode_string(&output)
}

fn main() {
    // Get the input string from command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_string>", args[0]);
        std::process::exit(1);
    }
    let result = match &args[1][..] {
        "sha3-224" => sha_224(&args[2]),
        "sha3-256" => sha_256(&args[2]),
        "sha3-384" => sha_384(&args[2]),
        "sha3-512" => sha_512(&args[2]),
        "shake128" => shake_128(&args[3], args[2].parse::<usize>().unwrap()),
        "shake256" => shake_256(&args[3], args[2].parse::<usize>().unwrap()),
        _ => {
            eprintln!("Unsupported hash algorithm: {}", args[1]);
            std::process::exit(1);
        }
    };
    // Print the hash in hexadecimal format
    println!("{}", result);
}
