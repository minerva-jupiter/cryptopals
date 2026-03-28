use base64::prelude::*;
use openssl::symm::{Cipher, decrypt};
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let b64_data = buffer.replace('\n', "").replace('\r', "");
    const KEY: &str = "YELLOW SUBMARINE";
    let decoded = BASE64_STANDARD.decode(b64_data.as_bytes())?;
    let decrypted = decode_aes(&decoded, KEY)?;
    print!("{}", decrypted);
    Ok(())
}
fn decode_aes(input: &[u8], key_str: &str) -> Result<String, Box<dyn std::error::Error>> {
    let key = key_str.as_bytes();
    let iv: Option<&[u8]> = None;
    let cipher = Cipher::aes_128_ecb();
    let decoded = decrypt(cipher, key, iv, input)?;
    Ok(String::from_utf8(decoded)?)
}
