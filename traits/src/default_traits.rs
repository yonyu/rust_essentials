#[derive(Debug)]
struct Satellite {
    name: String,
    velocity: f64 // miles per second
}

#[derive(Debug)]
struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: u32 // miles
}

trait Description {
    fn describe(&self) -> String {
        String::from("an object flying through space!")
    }
}

impl Description for Satellite {

}

impl Description for SpaceStation {
    fn describe(&self) -> String {
        format!("The {} is {} miles high with {} crew members.", self.name, self.altitude, self.crew_size)
    }
}

#[cfg(test)]
mod tests {
    use std::mem;
    use super::*;
    #[test]
    fn test_traits() {
        let hubble = Satellite {
            name:  String::from("Hubble Telescope"),
            velocity: 4.72
        };

        let iss = SpaceStation {
            name: String::from("International Space Station"),
            crew_size: 7,
            altitude: 254
        };

        println!("hubble is {}", hubble.describe());
        println!("iss is {}", iss.describe());
    }
}