use crate::lifetime_annotation;

pub fn best_fuel<'a>(x: &'a str, y: &'a str) -> &'a str { // Using a named lifetime parameter
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn demonstrate_lifetime_annotation() {
    let result;
    let propellant1 = String::from("RP-1");
    //{
        let propellant2 = String::from("LNG");
        result = lifetime_annotation::best_fuel(&propellant1, &propellant2);
    //}
    println!("Best fuel: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_demo_borrow_checker() {
        demonstrate_lifetime_annotation();
    }
}