pub mod filters;
pub mod utils;

use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use filters::bloom_filter::BloomFilter;
use utils::{arg_parser::Arguments, file_manager::save_to_db};

pub fn build_filter(arguments: &Arguments) -> Result<(), Box<dyn Error>> {
    let Arguments {
        build,
        probability,
        hash_functions,
        size,
        ..
    } = arguments;
    let path = build.clone().unwrap();

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let mut bloom_filter = BloomFilter::new(lines.len(), *probability, *size, *hash_functions);

    for line in lines {
        let mut word = line.as_bytes();
        bloom_filter.insert(&mut word);
    }

    let filter_results = bloom_filter.get_result();

    save_to_db(filter_results)
}

pub fn load_filter() -> Result<BloomFilter, Box<dyn Error>> {
    let results = utils::file_manager::read_from_db()?;

    Ok(BloomFilter::from(results))
}
