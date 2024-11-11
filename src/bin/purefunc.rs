// pure functions
// pure functions are always preferred because it produces the same output along as input is the same
// cargo add sha256

use sha256::digest;

fn generate_sha_256_hash(input: &str) -> String {
    let hash = digest(input.as_bytes());

    hash
}

fn main () {
    let input = "Hello, world!";
    let hash = generate_sha_256_hash(input);

    println!("The SHA-256 hash of '{}' is: {}", input, hash);
    
}

