use std::error::Error;

use spell_checker::{build_filter, load_filter, utils::arg_parser::setup_parser};

fn main() -> Result<(), Box<dyn Error>> {
    let arguments = setup_parser()?;

    if let Some(ref _build) = arguments.build {
        build_filter(&arguments)
    } else {
        let filter = load_filter()?;
        let mut missing_words = Vec::new();
        for word in arguments.words {
            let word_lower_case = word.to_ascii_lowercase();
            if !filter.lookup(&mut word_lower_case.as_bytes()) {
                missing_words.push(word);
            }
        }

        println!("Missing words: {:?}", missing_words);

        Ok(())
    }
}
