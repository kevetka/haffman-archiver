use std::io;

mod encoder;
mod tree;

use crate::encoder::count_frequencies;
use crate::tree::build_huffman_tree;

fn main() -> io::Result<()> {
    let test_filename = "test_input.txt";
    std::fs::write(test_filename, "Lorem ipsum dolor sit amet ut deserunt culpa id sunt dolore labore ex officia. Qui pariatur eiusmod pariatur cillum ut dolore exercitation in ad elit. Ex est irure minim aliquip. Anim sint in exercitation reprehenderit cupidatat magna velit. Nostrud pariatur proident ad exercitation.")?;

    println!("-- Этап 1: Подсчет частот --");
    let frequencies = count_frequencies(test_filename).unwrap();

    for (byte, &freq) in frequencies.iter().enumerate() {
        if freq > 0 {
            println!("Символ '{}' ({}) в количестве {}", byte as u8 as char, byte, freq);
        }
    }

    println!("-- Этап 2: Построение дерева Хаффмана --");
    let root = build_huffman_tree(frequencies);

    match root {
        Some(root_node) => {
            println!("Дерево успешно построено. Вес корня: {}", root_node.weight);
        }
        None => {
            println!("Файл пуст, дерево не построено.");
        }
    }

    std::fs::remove_file("test_input.txt").unwrap();
    
    Ok(())
}