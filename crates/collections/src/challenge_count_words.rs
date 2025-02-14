use std::fs;
use std::collections::HashMap;

pub fn count_words(file_name: &str) {
    // Read in a text file
    // Split the text into words
    // Count the number of time each word appears
    // Print a message with the most common words and how many times they appear

    // Read in a text file (Gutenberg)
    let test = match fs::read_to_string(file_name) {
        Ok(text) => text.to_lowercase(),
        Err(error) => {
            println!("Error reading file: {}", error);
            std::process::exit(2);
        }
    };

    // Split the text into words
    let words = test.split_whitespace().collect::<Vec<&str>>();
    // Count the number of time each word appears
    let mut word_count = HashMap::new();
    for word in words {
        // remove the last char form word if it is a punctuation
        let word = word.trim_end_matches  (|c| c == '.' || c == ',' || c == '!' || c == '?' || c == ':' || c == ']' || c == ')' || c == '_');
        let word = word.trim_start_matches(|c| c == '.' || c == ',' || c == '!' || c == '?' || c == ':' || c == '[' || c == '(' || c == '_');
        if word.is_empty() {
            continue;
        }

        // convert word to lowercase
        let count = word_count.entry(word.to_lowercase()).or_insert(0);

        *count += 1;
    }
    // Print a message with the most common words and how many times they appear
    let mut max_count = 0;
    let mut max_word = "";
    for (word, count) in &word_count {
        if *count > max_count {
            max_count = *count;
            max_word = word;
        }
    }

    println!("The most common word is '{}' and it appears {} times", max_word, max_count);
}

fn count_words_mine(file_name: &str) {
    // Read in a text file
    // Split the text into words
    // Count the number of time each word appears
    // Print a message with the most common words and how many times they appear

    // Read in a text file (Gutenberg)
    let test = fs::read_to_string(file_name).expect("Failed to read file");
    // Split the text into words
    let words = test.split_whitespace();
    // Count the number of time each word appears
    let mut word_count = HashMap::new();
    for word in words {
        // remove the last char form word if it is a punctuation
        let word = word.trim_end_matches  (|c| c == '.' || c == ',' || c == '!' || c == '?' || c == ':' || c == ']' || c == ')' || c == '_');
        let word = word.trim_start_matches(|c| c == '.' || c == ',' || c == '!' || c == '?' || c == ':' || c == '[' || c == '(' || c == '_');
        if word.is_empty() {
            continue;
        }

        // convert word to lowercase
        let count = word_count.entry(word.to_lowercase()).or_insert(0);

        *count += 1;
    }
    // Print a message with the most common words and how many times they appear
    let mut max_count = 0;
    let mut max_word = "";
    for (word, count) in &word_count {
        if *count > max_count {
            max_count = *count;
            max_word = word;
        }
    }

    println!("The most common word is '{}' and it appears {} times", max_word, max_count);
}