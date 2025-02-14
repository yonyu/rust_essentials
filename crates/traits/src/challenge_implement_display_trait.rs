use std::fmt;
//use std::fmt::Formatter;

// pub trait Display {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;
// }

struct Satellite {
   name: String,
   velocity: f64,
}

impl fmt::Display for Satellite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Satellite: {}, velocity: {:.1}", self.name, self.velocity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_display_traits() {
        let s = Satellite {
            name: String::from("ISS"),
            velocity: 90.0,
        };
        assert_eq!(format!("the satellite is : {}", s), "the satellite is : Satellite: ISS, velocity: 90.0");
    }
}