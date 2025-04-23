use sha2::{Digest, Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256};

fn sha_224(input_string: &str) -> String {
    let result = Sha224::digest(input_string.as_bytes());
    format!("{:x}", result)
}

fn sha_256(input_string: &str) -> String {
    let result = Sha256::digest(input_string.as_bytes());
    format!("{:x}", result)
}

fn sha_384(input_string: &str) -> String {
    let result = Sha384::digest(input_string.as_bytes());
    format!("{:x}", result)
}

fn sha_512(input_string: &str) -> String {
    let result = Sha512::digest(input_string.as_bytes());
    format!("{:x}", result)
}

fn sha_512_224(input_string: &str) -> String {
    let result = Sha512_224::digest(input_string.as_bytes());
    format!("{:x}", result)
}

fn sha_512_256(input_string: &str) -> String {
    let result = Sha512_256::digest(input_string.as_bytes());
    format!("{:x}", result)
}

fn main() {
    // Get the input string from command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_string>", args[0]);
        std::process::exit(1);
    }
    let input_string = &args[2];

    let result = match &args[1][..] {
        "sha224" => sha_224(input_string),
        "sha256" => sha_256(input_string),
        "sha384" => sha_384(input_string),
        "sha512" => sha_512(input_string),
        "sha512_224" => sha_512_224(input_string),
        "sha512_256" => sha_512_256(input_string),
        _ => {
            eprintln!("Unsupported hash function: {}", args[1]);
            std::process::exit(1);
        }
    };

    // Print the hash in hexadecimal format
    println!("{}", result);
}
