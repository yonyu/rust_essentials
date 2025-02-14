use std::any;
use std::fmt;

fn print_type<T: fmt::Debug>(item:  T) { // fmt::Display
    println!("{:?} is {}", item, any::type_name::<T>());
}

#[cfg(test)]
mod tests {
    //use std::mem;
    use super::*;
    #[test]
    fn test_traits() {
        print_type(13);
        print_type(13.0);
        print_type("thirteen");
        print_type([13]);
    }
}