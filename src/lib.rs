pub mod utils {
    pub fn read_buffer() -> Result<String, Box<dyn std::error::Error>> {
        let mut buffer = String::new();
        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line.");
        Ok(buffer.trim().to_string())
    }
}
