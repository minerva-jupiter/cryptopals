use cryptopals::utils::read_buffer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter first buffer");
    let first = hex_to_bytes(&read_buffer()?);
    println!("Enter second buffer");
    let second = hex_to_bytes(&read_buffer()?);
    if first.len() != second.len() {
        return Err("buffer length is not matched!".into());
    }
    println!("xor answer is {}", bytes_to_hex(&xor(&first, &second)));
    Ok(())
}
fn hex_to_bytes(hex: &str) -> Vec<u8> {
    hex.as_bytes()
        .chunks(2)
        .map(|chunk| {
            let s = std::str::from_utf8(chunk).unwrap();
            u8::from_str_radix(s, 16).expect("Invalid hex")
        })
        .collect()
}
fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
fn xor(first: &[u8], second: &[u8]) -> Vec<u8> {
    first
        .iter()
        .zip(second.iter())
        .map(|(a, b)| a ^ b)
        .collect()
}
