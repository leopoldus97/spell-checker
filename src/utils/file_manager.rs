use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read, Write},
};

use crate::filters::bloom_filter::FilterResults;

const FILE_PATH: &str = "db/words.bf";
const IDENTIFIER: &[u8; 4] = b"CCBF";
const VERSION: u16 = 1;

pub fn read_from_db() -> Result<FilterResults, Box<dyn Error>> {
    let file = File::open(FILE_PATH)?;
    let mut reader = BufReader::new(file);
    let mut identifier = [0; 4];
    let mut version = [0; 2];
    let mut number_of_hash_functions = [0; 2];
    let mut bit_array_size = [0; 4];
    let mut probability = [0; 4];
    let mut buffer = Vec::new();

    reader.read_exact(&mut identifier)?;
    reader.read_exact(&mut version)?;
    reader.read_exact(&mut number_of_hash_functions)?;
    reader.read_exact(&mut bit_array_size)?;
    reader.read_exact(&mut probability)?;

    if &identifier != IDENTIFIER {
        return Err("Identifier mismatch".into());
    }

    if version != VERSION.to_be_bytes() {
        return Err("Version mismatch".into());
    }

    reader.read_to_end(&mut buffer)?;

    let mut bitset = Vec::new();
    for bit in buffer {
        bitset.push(bit == 1);
    }

    Ok(FilterResults::new(
        bitset,
        u16::from_be_bytes(number_of_hash_functions) as u8,
        u32::from_be_bytes(bit_array_size) as usize,
        f32::from_be_bytes(probability),
    ))
}

pub fn save_to_db(filter_results: FilterResults) -> Result<(), Box<dyn Error>> {
    let identifier = IDENTIFIER.map(|b| b.to_be());
    let version = VERSION.to_be_bytes();
    let number_of_hash_functions = filter_results.get_number_of_hash_functions();
    let bit_array_size = filter_results.get_array_size();
    let probability = filter_results.get_probability();

    let mut bytes: Vec<u8> = Vec::new();
    bytes.extend(&identifier);
    bytes.extend(&version);
    bytes.extend(&number_of_hash_functions);
    bytes.extend(&bit_array_size);
    bytes.extend(&probability);
    bytes.extend(filter_results.get_bitset());

    let mut file = File::create(FILE_PATH)?;
    file.write_all(&bytes)?;
    Ok(())
}
