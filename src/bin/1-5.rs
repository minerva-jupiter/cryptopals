fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter key:");
    let mut key = String::new();
    std::io::stdin().read_line(&mut key)?;
    let key_bytes = key.into_bytes();
    let mut crypted: Vec<Vec<u8>> = Vec::new();
    for _ in 0..2 {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        crypted.push(xor(&input.into_bytes(), &key_bytes));
    }
    println!("");
    for c in &crypted {
        println!("{}", bytes_to_hex(c));
    }
    Ok(())
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
fn xor(first: &[u8], second: &[u8]) -> Vec<u8> {
    first
        .iter()
        .zip(second.iter().cycle())
        .map(|(a, b)| a ^ b)
        .collect()
}
