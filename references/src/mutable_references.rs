#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process_fuel() {
        process_fuel();
    }
}

fn process_fuel() {
    let mut propellant = String::from("RP-1");

    process_propellant(&mut propellant);
    println!("propellant: {}", propellant);
}

/*******************************************************************
 * When this function is called, propellant will reference to the variable
 * that owns the value.
 */
fn process_propellant(propellant: &mut String) {
    propellant.push_str("*");
    println!("processing propellant: {}", propellant);
}
