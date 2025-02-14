fn print_messages() {
    let message = String::from("Greetings from Earth!");
    println!("Message is {}", message);
    // let first_word = &message[..5];
    let last_word = &message[15..15+5];
    println!("Last word is {}", last_word);

    // array slices
    let planets = ["Mercury", "Venus", "Earth", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune"]; // sorry, Pluto (not a full size planet)!
    let inner_planets: &[&str] = &planets[..4];
    println!("Inner planets are: {:?}", inner_planets);

    let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
    let partial_numbers = &numbers[..4];
    println!("Partial numbers are: {:?}", partial_numbers);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_messages() {
        print_messages();
        assert_eq!(1, 1);
    }
}