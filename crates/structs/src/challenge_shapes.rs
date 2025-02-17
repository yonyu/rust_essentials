#[warn(dead_code)]
struct Rectangle {
    width: f64,
    height: f64,
}

#[warn(dead_code)]
impl Rectangle {
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }

    fn get_area(&self) -> f64 {
        self.width * self.height
    }

    fn get_perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn scale(&mut self, scale: f64) {
        self.width *= scale;
        self.height *= scale;
    }
}

#[warn(dead_code)]
fn practice() {
    let mut rect = Rectangle::new(10.0, 20.0);

    println!("Area: {}", rect.get_area());
    println!("Perimeter: {}", rect.get_perimeter());

    rect.scale(2.0);

    println!("Area: {}", rect.get_area());
    println!("Perimeter: {}", rect.get_perimeter());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rectangle() {
        practice();

        let mut rect = Rectangle::new(1.2, 3.4);
        assert_eq!(rect.get_area(), 4.08);
        rect.scale(0.5);
        assert_eq!(rect.get_area(), 1.02);
    }
}