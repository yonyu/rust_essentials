#[derive(Debug)] // derives the trait Debug
#[derive(Clone)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons;
    }
}

fn practice() {
    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 0.0, // Gallon
    };

    let name = vehicle.get_name();

    println!("vehicle_name: {}", name);
    println!("propellant is: {}", vehicle.propellant);
    vehicle.add_fuel(1000.0);
    println!("propellant after adding is: {}", vehicle.propellant);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_practice() {
        practice();
        assert_eq!(2 + 2, 4);
    }
}