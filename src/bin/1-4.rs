use cryptopals::utils::read_buffer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut keys: Vec<u8> = Vec::new();
    loop {
        let input = read_buffer()?;
        if input.is_empty() {
            break;
        }
        keys.push(detect_key(&hex_to_bytes(&input))?);
    }
    println!("keys: {}", String::from_utf8_lossy(&keys));
    Ok(())
}

fn detect_key(crypted: &[u8]) -> Result<u8, Box<dyn std::error::Error>> {
    let (key, msg, score) = crack_single_byte_xor(crypted);
    // println!(
    //     "key: {}, msg: {}, score: {}",
    //     bytes_to_hex(&[key]),
    //     bytes_to_hex(&xor(&crypted, &[key])),
    //     score
    // );
    println!("{}", String::from_utf8_lossy(&[key]));
    Ok(key)
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
        .zip(second.iter().cycle())
        .map(|(a, b)| a ^ b)
        .collect()
}
fn score_text(bytes: &[u8]) -> f64 {
    let mut score = 0.0;
    for &b in bytes {
        match b.to_ascii_lowercase() {
            b'e' => score += 12.02,
            b't' => score += 9.10,
            b'a' => score += 8.12,
            b'o' => score += 7.68,
            b'i' => score += 7.31,
            b'n' => score += 6.95,
            b's' => score += 6.28,
            b'r' => score += 6.02,
            b'h' => score += 5.92,
            b'd' => score += 4.32,
            b'l' => score += 3.98,
            b'u' => score += 2.88,
            b' ' => score += 15.0,
            0..=31 | 127 => score -= 50.0,
            32..=126 => score += 1.0,
            _ => score -= 10.0,
        }
    }
    score
}

fn crack_single_byte_xor(crypted: &[u8]) -> (u8, Vec<u8>, f64) {
    let mut best_score = f64::MIN;
    let mut best_key = 0;
    let mut best_msg = Vec::new();

    for key in 0..=255 {
        let decrypted: Vec<u8> = crypted.iter().map(|&b| b ^ key).collect();
        let score = score_text(&decrypted);

        if score > best_score {
            best_score = score;
            best_key = key;
            best_msg = decrypted;
        }
    }
    (best_key, best_msg, best_score)
}
