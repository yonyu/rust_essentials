/*****************************************************************************
 * Trim leading and trailing whitespace from a string.
 *****************************************************************************/
pub fn trim_spaces(s: &str) -> &str {
    let mut start = s.len();
    // iterate directly over chars, better than over bytes
    for (index, item) in s.chars().enumerate() {
        if item != ' ' {
            start = index;
            break;
        }
    }

    // if  s.len() == start {  // all are whitespaces
    //     return &s[0..0];
    // }

    let mut end = s.len();
    for (index, item) in s.chars().rev().enumerate() {
        if item != ' ' {
            end = s.len() - index;
            break;
        }
    }

    &s[start..end]
}

/*****************************************************************************
 * Trim leading and trailing whitespace from a string.
 *****************************************************************************/
pub fn trim_spaces_trial(s: &str) -> String {
    let chars = s.as_bytes();
    let mut left_trimmed =  String::new();
    let mut start = 0;
    for (index, &item) in chars.iter().enumerate() {
        if item != 32 {
            left_trimmed.push_str(&s[start..]);
            break;
        }
        start += 1;
    }

    if  left_trimmed.len() == 0 {
        return left_trimmed;
    }

    let mut trimmed =  String::new();
    let chars = left_trimmed.as_bytes();
    start = left_trimmed.len();
    for (index, &item) in chars.iter().rev().enumerate() {
        if item != 32 {
            trimmed.push_str(&left_trimmed[..start]);
            break;
        }
        start -= 1;
    }

    trimmed
}

fn trim() {
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");

    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");

    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");

    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");

    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");

    let test6 = "";
    assert_eq!(trim_spaces(test6), "");

    let test7 = " x🚀xx ";
    assert_eq!(trim_spaces(test7), "x🚀xx");
    println!("Tests passed!");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_trim() {
        trim();
    }

}