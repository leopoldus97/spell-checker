use std::error::Error;

use clap::{Arg, Command};

use crate::filters::bloom_filter::Probability;

pub struct Arguments {
    pub build: Option<String>,
    pub probability: Option<Probability>,
    pub hash_functions: Option<u8>,
    pub size: Option<usize>,
    pub words: Vec<String>,
}

pub fn setup_parser() -> Result<Arguments, Box<dyn Error>> {
    let matches = Command::new("Spell checker")
        .version("0.1.0")
        .about("Program for checking whether a word is in a dictionary or not")
        .args([
            Arg::new("build")
                .short('b')
                .long("build")
                .value_name("FILE")
                .help("Builds the database for the filter"),
            Arg::new("probability")
                .short('p')
                .long("probability")
                .value_name("f32")
                .help("Sets the expected probability of false positives. Default is 0.2"),
            Arg::new("hash-functions")
                .short('H')
                .long("hash-functions")
                .value_name("u8")
                .help("Sets the number of hash functions to use. Default is calculated based on the size of the bit array and the number of elements inserted into the filter"),
            Arg::new("size")
                .short('s')
                .long("size")
                .value_name("usize")
                .help("Sets the size of the bit array. Default is calculated based on the number of elements inserted into the filter and the expected probability of false positives"),
            Arg::new("words")
                .help("Words you like to check against the database")
                .num_args(1..)
                .conflicts_with_all(["build", "probability", "hash-functions", "size"]),
        ])
        .get_matches();

    let build = matches
        .get_one::<String>("build")
        .map(|build| build.to_string());
    let probability = matches
        .get_one::<f32>("probability")
        .map(|probability| Probability {
            value: *probability,
        });
    let hash_functions = matches.get_one::<u8>("hash-functions").cloned();
    let size = matches.get_one::<usize>("size").copied();
    let words: Vec<String> = matches
        .get_many::<String>("words")
        .unwrap_or_default()
        .map(|words: &String| words.to_string())
        .collect();

    Ok(Arguments {
        build,
        words,
        probability,
        hash_functions,
        size,
    })
}
