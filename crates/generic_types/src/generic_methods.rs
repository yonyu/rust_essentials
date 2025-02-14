#[derive(Debug)]
struct Rectangle<T, U> {
    width: T,
    height: U,
}

impl<T, U> Rectangle<T, U> {
    fn new(width: T, height: U) -> Rectangle<T, U> {
        Rectangle { width, height }
    }

    fn get_width(&self) -> &T { // we don't know what type might be. assume the type doesn't implement copy
        &self.width
    }
    // fn get_area(&self) -> T {
    //     &self.width * &self.height
    // }
}

impl Rectangle<u8, u8> {
    fn get_perimeter(&self) -> u8 {
        2 * (self.width + self.height)
    }
}

fn practice() {
    let mut rect = Rectangle::new(10u16, 20u32);

    eprintln!("rect is {:?}", rect);
    println!("Width: {}", rect.get_width());

    // println!("Area: {}", rect.get_area());
    // println!("Perimeter: {}", rect.get_perimeter());
    //
    // rect.scale(2.0);
    //
    // println!("Area: {}", rect.get_area());
    // println!("Perimeter: {}", rect.get_perimeter());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rectangle() {
        practice();

        let mut rect = Rectangle::new(1.2, 3.4);
        // assert_eq!(rect.get_area(), 4.08);
    }
}
