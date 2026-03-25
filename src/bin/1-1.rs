use cryptopals::utils::read_buffer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hex_input = read_buffer()?;
    let bytes = hex_to_bytes(&hex_input);
    let base64_output = bytes_to_base64(&bytes);

    println!("{}", base64_output);

    Ok(())
}

const BASE64_TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    hex.as_bytes()
        .chunks(2)
        .map(|chunk| {
            let s = std::str::from_utf8(chunk).unwrap();
            u8::from_str_radix(s, 16).expect("Invalid hex")
        })
        .collect()
}

fn bytes_to_base64(bytes: &[u8]) -> String {
    let mut result = String::with_capacity((bytes.len() + 2) / 3 * 4);

    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as usize;
        let b1 = chunk.get(1).map(|&b| b as usize).unwrap_or(0);
        let b2 = chunk.get(2).map(|&b| b as usize).unwrap_or(0);

        let n = (b0 << 16) | (b1 << 8) | b2;

        result.push(BASE64_TABLE[(n >> 18) & 0x3F] as char);
        result.push(BASE64_TABLE[(n >> 12) & 0x3F] as char);

        if chunk.len() > 1 {
            result.push(BASE64_TABLE[(n >> 6) & 0x3F] as char);
        } else {
            result.push('=');
        }

        if chunk.len() > 2 {
            result.push(BASE64_TABLE[n & 0x3F] as char);
        } else {
            result.push('=');
        }
    }
    result
}
