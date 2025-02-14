#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process_fuel() {
        process_fuel();
    }
}

fn process_fuel() {
    let mut propellant = process_propellant();
    println!("propellant: {}", propellant);
}

/*******************************************************************
 * When this function is called, the receiving variable
 * will take the ownership of the value returned.
 */
fn process_propellant() -> String {
    let propellant = String::from("RP-1");
    println!("processing propellant: {}", propellant);

    propellant
}

/******************************************************************************
 * When this function is called, propellant will reference to the variable
 * that owns the value:
 * returns a reference to data owned by the current function
 *****************************************************************************/
/*
fn process_propellant_dangling() -> &String {
    let propellant = String::from("RP-1");
    println!("processing propellant: {}", propellant);

    &propellant // reference to a local variable; it will be dropped when the function finishes
}
*/
