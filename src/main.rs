use std::process::exit;
use ed25519_dalek::{Signature, VerifyingKey};
use memfd_exec::MemFdExecutable;

const PUBLIC_KEY: [u8; 32] = [50, 63, 45, 139, 167, 192, 113, 241, 60, 197, 168, 81, 207, 145, 183, 252, 249, 93, 20, 96, 157, 27, 86, 83, 253, 122, 198, 141, 71, 122, 188, 124];
const SIGNATURE: [u8; 64] = [0; 64];
const BUNDLED_APP: &[u8] = include_bytes!("app.o");

fn main() {
    let public_key = VerifyingKey::from_bytes(&PUBLIC_KEY).expect("Can't create VerifyingKey from bytes");
    let signature = Signature::from_bytes(&SIGNATURE);
    
    if public_key.verify_strict(BUNDLED_APP, &signature).is_err() {
        println!("Wrong signature");
        exit(-1);
    }

    MemFdExecutable::new("app", BUNDLED_APP).spawn().expect("Can't spawn app");
}
