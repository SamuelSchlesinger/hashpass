fn readable(bytes: &[u8]) -> String {
    let mut password = String::new();
    for byte in bytes {
        let character = (byte % (94 - 21) + 21) as char;
        password.push(character);
    }
    password
}

fn hashpass(password: String, service: String) -> String {
    use sha3::Digest;
    let mut hasher = sha3::Keccak256::default();
    hasher.update(&password);
    readable(hasher.finalize().as_slice())
}

fn main() {
    let mut args = std::env::args();
    println!(
        "{}",
        hashpass(
            args.next()
                .expect("First argument must be your master password"),
            args.next()
                .expect("Second argument must be the URL of the service")
        )
    );
}
