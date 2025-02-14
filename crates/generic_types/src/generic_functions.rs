fn get_biggest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rectangle() {
        println!("biggest is {}", get_biggest(3, 4));
        println!("biggest is {}", get_biggest(3.2, 4.5));
        // let mut rect = Rectangle::new(1.2, 3.4);
        // assert_eq!(rect.get_area(), 4.08);
        // rect.scale(0.5);
        // assert_eq!(rect.get_area(), 1.02);
    }
}