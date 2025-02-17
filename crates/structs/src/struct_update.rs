#[warn(dead_code)]
#[derive(Debug)] // derives the trait Debug
#[derive(Clone)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

#[warn(dead_code)]
fn practice() {
    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 835958.8, // Gallon
    };

    let vehicle2 = Shuttle {
        //name: String::from("Discovery"),
        //..vehicle // after vehicle.name is assigned to vehicle2.name, vehicle2.name takes the ownership of the string value "Endeavor"
        ..vehicle.clone()
    };

    vehicle.crew_size = 8;
    println!("vehicle: {:?}", vehicle);   // 'Debug' formatting
    println!("vehicle2: {:?}", vehicle2);
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