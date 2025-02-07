#[derive(Debug)]
pub enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64) // length of each side
}

pub enum  Command {
    Clear,
    Drawline(f64, f64), // x, y
    DrawShape(Shape)
}