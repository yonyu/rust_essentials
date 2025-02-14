//use std::any;
use std::fmt;

//fn compare_and_print<T: fmt::Display + PartialEq + From<U>, U: fmt::Display + PartialEq + Copy>(a: T, b: U) { // fmt::Display
fn compare_and_print<T, U>(a: T, b: U)
    where T: fmt::Display + PartialEq + From<U>,
          U: fmt::Display + PartialEq + Copy
{ // fmt::Display
    if a == T::from(b) {
        println!("{} is equal to {}", a, b);
    } else {
        println!("{} is NOT equal to {}", a, b);
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_traits() {
        compare_and_print(1.0, 1);
        compare_and_print(1.1, 1);

    }
}