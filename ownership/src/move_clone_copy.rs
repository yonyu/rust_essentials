
fn moving() {
    let outer_planet: String;
    {
        let mut inner_planet = String::from("Mercury");
        outer_planet = inner_planet; // move: ownership of inner_planet is moved to outer_planet
        //println!("inner_planet is {}", inner_planet);  // value bound to inner_planet is moved

        let mut x = 5;
        let y = x; // copy: x is copied to y
        x = x + 1;
        println!("x is {}", x);
        println!("y is {}", y);
    }
    println!("outer_planet is {}", outer_planet);
}

fn cloning() {
    let outer_planet: String;
    {
        let mut inner_planet = String::from("Mercury");
        outer_planet = inner_planet.clone(); // created the second copy of the value bound by inner_planet
        inner_planet.clear();
        println!("inner_planet is {}", inner_planet);  // empty
    }
    println!("outer_planet is {}", outer_planet);

}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]

    fn test_moving() {
        moving();
    }

    #[test]
    fn test_cloning() {
        cloning();
    }
}