use sha2::{Sha256, Digest};
use std::io;

fn main() {

    println!("Enter a password");

    let mut msg = String::new();

    io::stdin()
        .read_line(&mut msg)
        .expect("failed to read line");
    
    let mut hash = Sha256::new();

    hash.update(&mut msg);
    let result = hash.finalize();

    println!("{:x}", result);
}
