mod caeser_cypher_encrypt;
mod caeser_cypher_decrypt;
mod password_generation;
mod setup;

pub use crate::caeser_cypher_decrypt::caeser_decrypt;
pub use crate::password_generation::password;
pub use crate::caeser_cypher_encrypt::caeser_encrypt;
pub use crate::setup::setup;

use sha2::{Sha256, Digest};
use std::path::Path;
//use std::io::stdin;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if &args[1] == "--setup" {
        setup();
    }

    else {
        let mut line = String::new();
        println!("Enter your masterpassword");
        println!("> ");
        let input: usize = std::io::stdin().read_line(&mut line).unwrap();
        let input: String = input.to_string();
        let unencrypted_string = &input[..];
    }
    

}
