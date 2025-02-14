fn process_fuel() {
    let mut car_fuel = 1;
    crate::transfer_ownership::process_value_on_stack(car_fuel); // the value will be copied to the argument. no move occurs!
    println!("car_fuel: {}", car_fuel);

    let mut propellant = String::from("RP-1");
    //propellant =
    //process_value_on_heap(propellant); // move occurs. the ownership of the value is moved to the function argument.
    println!("propellant: {}", propellant);
}

fn process_value_on_stack(mut propellant: i32) {
    propellant = propellant + 1;
    println!("processing gas: {}", propellant);
}

/*******************************************************************
 * When this function is called, the ownership of the value is moved
 * to the parameter propellant.
 * After the function call, the ownership of the value is discarded.
 * The string value will be dropped from memory.
 */
fn process_value_on_heap(mut propellant: String) {
    propellant.push_str("*");
    println!("processing propellant: {}", propellant);

    // propellant
}

fn process_fuel_by_ref() {
    let mut car_fuel = 1;
    process_value_on_stack_by_ref(&mut car_fuel);
    println!("car_fuel: {}", car_fuel);

    let mut rocket_fuel = String::from("RP-1");
    process_value_on_heap_by_ref(&mut rocket_fuel);
    println!("rocket fuel: {}", rocket_fuel);
}

fn process_value_on_stack_by_ref(propellant: &mut i32) {
    *propellant = *propellant + 1;
    println!("processing car gas: {}", propellant);
}

fn process_value_on_heap_by_ref(propellant: &mut String) {
    (*propellant).push_str("1");
    println!("processing propellant: {}", propellant);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process_fuel() {
        process_fuel();
    }

    #[test]
    fn test_process_fuel_by_ref() {
        process_fuel_by_ref();
    }
}