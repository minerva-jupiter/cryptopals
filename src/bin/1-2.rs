use cryptopals::utils::read_buffer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter first buffer");
    let first = read_buffer()?;
    println!("Enter second buffer");
    let second = read_buffer()?;
    if first.len() != second.len() {
        return Err("buffer length is not matched!".into());
    }
    Ok(())
}
