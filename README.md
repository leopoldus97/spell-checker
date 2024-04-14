# Coding Challenge: `Spell Checker`

This application was created as part of a coding challenge. The challenge details can be found [here](https://codingchallenges.fyi/challenges/challenge-bloom).

The following resources were used for implementing and understanding the functionality of Bloom Filters and MurmurHash:

- [Geeks For Geeks tutorial](https://www.geeksforgeeks.org/bloom-filters-introduction-and-python-implementation/)
- [Wikipedia - Bloom filter](https://en.wikipedia.org/wiki/Bloom_filter)
- [Wikipedia - MurmurHash](https://en.wikipedia.org/wiki/MurmurHash)

## Description

The Spell Checker application is designed to compare each word in the given text against a dictionary of correctly spelled words. If a word is not found in the dictionary, it is considered misspelled. The application then displays a list of all the misspelled words found in the text.

## How to Run

To run the Spell Checker application, follow these steps:

1. Clone the repository.
2. Run the application using the command `cargo build --release`. This will generate the executable file in the `target/release` directory called `spell-checker`.
3. Generate the dictionary file by running the application with the following argument `--build <path-to-word-file>`. For example, `./spell-checker --build /usr/share/dict/words`.
4. Run the application file and provide the text you want as argument to check for spelling errors.
5. The application will display the misspelled words, if any.

## Arguments
You can run the following arguments with the application:
- `spell-checker --build <path-to-word-file>`: This argument is used to generate the dictionary file. The dictionary file is used to compare the words in the text file to check for spelling errors.
- `spell-checker [words]...`: This argument is used to provide the words you want to check for spelling errors.
- `spell-checker --help`: This argument is used to display the help message.

## Notes
Currently the application only accepts text files with one word per line. If you would like to use a different format, you need to update the `read_file` function in the `main.rs` file.
