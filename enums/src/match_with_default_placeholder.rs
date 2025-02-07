pub fn match_with_default(x: u8) -> String {
    let result = match x {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => {
            println!("{} did not match", x);
            "something else"
        }
    };
    println!("result is {}", result);

    String::from(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_match_with_default_placeholder() {
        let result = match_with_default(2);
        assert_eq!(result, "two");

        let result = match_with_default(4);
        assert_eq!(result, "something else");
    }
}

