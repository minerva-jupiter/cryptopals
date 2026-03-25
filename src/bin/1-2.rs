fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter first buffer");
    let first = std::read_line()?;
    println!("Enter second buffer");
    let second = std::read_line()?;
    if first.len() != second.len() {
        println!("buffer length is not matched!");
        return Err(Box::new(()) as Box<dyn std::error::Error>);
    }
    Ok(())
}
