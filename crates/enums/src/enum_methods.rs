pub enum Shape {
    Circle(f64), // radius
    Rectangle(f64, f64), // width and height
    Triangle(f64, f64, f64) // length of each side
}

impl Shape {
    pub fn get_perimeter(&self) -> f64 {
        match *self { // * is optional
            Shape::Circle(r) => 2.0 * std::f64::consts::PI * r,
            Shape::Rectangle(w, h) => 2.0 * (w + h),
            Shape::Triangle(a, b, c) => a + b + c
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::match_with_default_placeholder::match_with_default;
    use super::*;
    #[test]
    fn test_get_perimeter() {
        let circle = Shape::Circle(2.0);
        let rectangle = Shape::Rectangle(2.0, 3.0);
        let triangle = Shape::Triangle(3.0, 4.0, 5.0);
        assert_eq!(circle.get_perimeter(), 12.56);
        assert_eq!(rectangle.get_perimeter(), 10.0);
        assert_eq!(triangle.get_perimeter(), 12.0);
    }
}