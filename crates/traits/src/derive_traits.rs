#[derive(PartialOrd, PartialEq)]
struct Satellite {
    name: String,
    velocity: f64 // miles per second
}

#[cfg(test)]
mod tests {
    //use std::mem;
    use super::*;
    #[test]
    fn test_traits() {
        let hubble = Satellite {
            name:  String::from("Hubble Telescope"),
            velocity: 4.72
        };

        let gps =  Satellite {
            name:  String::from("GPS"),
            velocity: 2.42
        };

        println!("hubble == gps is {}", hubble == gps);
        println!("hubble > gps is {}", hubble > gps);
    }
}