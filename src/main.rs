use std::io;

use crate::encoder::count_frequencies;

mod encoder;

fn main() -> io::Result<()> {
    let test_filename = "test_input.txt";
    std::fs::write(test_filename, "Hello world foirfiubhuiicaihfahfyvigvfhyhafvebgjuyuaihfjnrhfuizvvnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnndhhhdbvhswiojfehgujaihjfdvhjfdhjsfdvhjshjsvfdhjsvfbhjshjurehu34uy547845h3uf783fhw8y7q3w3ygf878hfuhgfhu48hfwuhi3yuf48u4rhebwghfwy4784ghuy4fwbfwyui7u84yfg5hubf4whuiy7ghu4jinhf!")?;

    println!("--Этап 1: Подсчет частот --");
    let frequencies = count_frequencies(test_filename).unwrap();

    for (byte, &freq) in frequencies.iter().enumerate() {
        if freq > 0 {
            println!("Символ '{}' ({}): {} раз", byte as u8 as char, byte, freq);
        }
    }

    Ok(())
}