mod writing_to_files;
mod reading_from_file;

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

    #[test]
    fn read_from_file() {

        // cargo.exe test --package input_output --lib tests::read_from_file -- --show-output
        reading_from_file::read_file("speech.txt");

    }

    #[test]
    fn write_to_file() {
        writing_to_files::write_speech();
    }
}
