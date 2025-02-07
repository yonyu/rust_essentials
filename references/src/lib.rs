mod borrowing_references;
mod mutable_references;
mod dangling_references;
mod slices;
mod slices_as_function_parameters;
mod trimming_leading_spaces;
mod multiple_references;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
