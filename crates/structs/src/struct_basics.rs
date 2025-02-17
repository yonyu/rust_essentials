#[warn(dead_code)]
#[derive(Debug)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}
#[warn(dead_code)]
fn practice() {
    let vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 835958.8, // Gallon
    };
    println!("vehicle.name: {}", vehicle.name);

    println!("vehicle: {:?}", vehicle);
}

#[warn(dead_code)]
fn update_vehicle() {
    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 835958.8, // Gallon
    };
    println!("vehicle.name: {}", vehicle.name);

    vehicle.name = String::from("Atlantis");
    println!("vehicle.name: {}", vehicle.name);

    println!("vehicle: {:?}", vehicle);  // 'Debug' formatting
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_update_vehicle() {
        update_vehicle();
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_practice() {
        practice();
        assert_eq!(2 + 2, 4);
    }
}