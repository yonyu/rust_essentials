pub fn demonstrate_borrowed_value_lifetime() {
    let propellant;
    {
        let rp1 = String::from("RP-1");
        propellant = &rp1;
        println!("propellant is {}", propellant);
    }
    //println!("propellant is {}", propellant);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_demo_borrow_checker() {
        demonstrate_borrowed_value_lifetime();
    }
}