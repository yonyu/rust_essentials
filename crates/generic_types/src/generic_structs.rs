#[derive(Debug)]
struct Rectangle<T> {
    width: T,
    height: T,
}

impl<T> Rectangle<T> {
    fn new(width: T, height: T) -> Rectangle<T> {
        Rectangle { width, height }
    }

    // fn get_area(&self) -> T {
    //     &self.width * &self.height
    // }

    // fn get_perimeter(&self) -> T {
    //     2.0 * (&self.width + &self.height)
    // }

    // fn scale(&mut self, scale: f64) {
    //     self.width *= &self.width * scale;
    //     self.height *= &self.height *scale;
    // }
}

fn practice() {
    let mut rect = Rectangle::new(10, 20);

    // println!("Area: {}", rect.get_area());
    // println!("Perimeter: {}", rect.get_perimeter());

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rectangle() {
        practice();

        // let mut rect = Rectangle::new(1.2, 3.4);
        // assert_eq!(rect.get_area(), 4.08);
        // rect.scale(0.5);
        // assert_eq!(rect.get_area(), 1.02);
    }
}
