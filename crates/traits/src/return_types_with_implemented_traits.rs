use std::fmt;

fn get_displayable() -> impl fmt::Display {
    // 13
    "Hello"
}

#[cfg(test)]
mod tests {
    //use std::mem;
    use super::*;
    #[test]
    fn test_traits() {
        println!("output is {}", get_displayable());
    }
}