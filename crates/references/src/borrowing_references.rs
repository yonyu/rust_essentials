#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process_fuel() {
        process_fuel();
    }
}

fn process_fuel() {
    let propellant = String::from("RP-1");
    //let (propellant, length) =
        //process_value_on_heap(propellant); // move occurs. the ownership of the value is moved to the function parameter.
    process_propellant(&propellant);
    println!("propellant: {}", propellant);
}

/*******************************************************************
 * When this function is called, propellant will reference to the variable
 * that owns the value.
 */
fn process_propellant(propellant: &String) {
    // propellant.push_str("*");
    println!("processing propellant: {}", propellant);
}

/*******************************************************************
 * When this function is called, the ownership of the value is moved
 * to the parameter propellant.
 * After the function call, return the variable that owns the value in the tuple.
 */
fn process_value_on_heap(propellant: String) -> (String, usize) {
    // propellant.push_str("*");
    println!("processing propellant: {}", propellant);
    let size = propellant.len();
    (propellant, size)
}
