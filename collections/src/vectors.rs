pub fn example() {
    // create a new vector which will start out empty without any elements
    let mut astronauts: Vec<String> = Vec::new();
    astronauts.push(String::from("Shepard"));
    astronauts.push(String::from("Grissom"));
    astronauts.push(String::from("Glenn"));
    println!("astronaut is {:?}", astronauts);

    // remove
    let last = astronauts.pop();
    println!("last is {:?}", last);

    // the vector owns the value it's storing,
    // we won't be able to pass the ownership of string to the third variable
    // Instead, we need to reference it using the borrow operator.
    //let third =  &astronauts[2];

    let third = astronauts.get(2);
    println!("third is {:?}", third); // None

    let countdown = vec![5, 4, 1, 2, 3];
    println!("countdown is {:?}", countdown);
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_example() {
        example();
        assert!(true);
    }

}