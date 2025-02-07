use mem;

struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

#[cfg(test)]
mod tests {
    use std::mem;
    use super::*;
    #[test]
    fn test_shuttle() {
        let mut vehicle = Shuttle {
            name: String::from("Atlantis"),
            crew_size: 7,
            propellant: 835958.0,
        };
        vehicle.name = String::from("Atlantis");
        println!("Vehicle size on stack is {} bytes", mem::size_of_val(&vehicle));

        let boxed_vehicle: Box<Shuttle> = Box::new(vehicle);
        println!("boxed_vehicle size on stack is {} bytes", mem::size_of_val(&boxed_vehicle));
        println!("boxed_vehicle size on heap is {} bytes", mem::size_of_val(&*boxed_vehicle));

        let unboxed_vehicle = *boxed_vehicle; // move boxed_vehicle back to stack from heap
        println!("unboxed_vehicle size on stack is {} bytes", mem::size_of_val(&unboxed_vehicle));
    }
}