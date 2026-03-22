use std::{fs::File, io::{BufReader, Read}};

pub fn count_frequencies(filename: &str) -> Result<[u64; 256], String> {
    let mut reader = BufReader::new(File::open(filename).map_err(|e| format!("Can not open file: {}", e))?);

    let mut frequencies = [0u64; 256];
    let mut buffer = [0u8; 4096];

    loop {
        let bytes_red = reader.read(&mut buffer).map_err(|e| format!("Can not read from file: {}", e))?;
        if bytes_red == 0 {
            break;
        }

        for &byte in &buffer[..bytes_red] {
            frequencies[byte as usize] += 1;
        }
    }

    Ok(frequencies)
}