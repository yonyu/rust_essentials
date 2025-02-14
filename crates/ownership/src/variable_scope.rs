// Reusing common variable names for different purposes
// is called shadowing.
//
// Shadowing is different from marking a variable as unused with the
// `_` identifier.
//
// The compiler will warn you when you shadow a variable.
//
// unintentionally shadowing variables can cause weird bugs
fn in_scope() {
    let planet= "Earth";

    //if true
    {
        //let planet = "Earth";
        println!("{}", planet);
    }
    println!("{}", planet);
}

fn shadowing_variables() {
    let planet = "Earth";
    println!("{}", planet);
    let planet = "Mars"; // shadowing the variable planet
    println!("{}", planet);
    let mut planet = 4;
    println!("{}", planet);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_in_scope() {
        in_scope();
    }

    #[test]
    fn test_shadowing_variables() {
        shadowing_variables();
    }
}