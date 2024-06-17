use std::path::Path;
use std::fs;
use ed25519_dalek::{ed25519::signature::SignerMut, SigningKey};

const PRIVATE_KEY: [u8; 32] = [79, 138, 237, 196, 27, 41, 3, 1, 216, 72, 140, 3, 112, 31, 47, 159, 95, 3, 252, 186, 205, 20, 6, 142, 146, 116, 34, 99, 128, 128, 125, 210];

fn main() {
    let main_rs = fs::read_to_string("src/main.rs").expect("Can't read main.rs");
    let app_o = fs::read("src/app.o").expect("Can't read app.o");

    let mut signing_key = SigningKey::from_bytes(&PRIVATE_KEY);
    let signature = signing_key.sign(&app_o);
    
    let replaced = main_rs.replace("[0; 64];", format!("{:?};", signature.to_vec()).as_str());

    fs::write(&Path::new("src/.main.rs"), &replaced).expect("Can't write new content to file");
}
