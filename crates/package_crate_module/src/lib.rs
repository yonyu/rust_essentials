

pub mod garden;
pub mod bar;
pub mod foo;
pub mod foobar;


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::garden::vegetables::Asparagus;
    
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works2() {
        let plant = Asparagus {};
        println!("{:?}", plant);
    }
}
