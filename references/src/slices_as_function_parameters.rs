/******************************************************************************
 * As a rule of thumb, when you’re writing functions to work with strings without
 * taking ownership, you should use the string slice data type for input and output
 * parameters, because it has the flexibility to also work with string references.
 * As a rule of thumb, when you’re writing functions to work with strings without
 * taking ownership, you should use the string slice data type for input and output
 * parameters, because it has the flexibility to also work with string references.
 ******************************************************************************/
fn print_messages() {
    let message = String::from("Greetings from Earth!");
    println!("The entire message is \"{}\"", message);
    let first_word = get_first_word_1(&message);
    println!("first word is \"{}\"", first_word);
    let first_word = get_first_word(&message[10..]);
    println!("second word is \"{}\"", first_word);
}

// string slice as argument
fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index]; // found a space
        }
    }

    &s // no spaces found; input is a single word
}

// string reference as argument
fn get_first_word_1(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index]; // found a space
        }
    }

    &s // no spaces found; input is a single word
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_messages() {
        print_messages();
        assert_eq!(1, 1);
    }
}