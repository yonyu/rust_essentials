pub fn convert_variables() {
    let x: i32;
    let s = "42";
    // The syntax parse::<i32>() is known as a turbofish in the Rust programming language.
    // The turbofish is a syntax that allows you to specify the wanted type of generic function.
    // The name "turbofish" comes from the resemblance of the syntax ::<> to a fish with a turbo
    // or rocket shape.
    x = s.parse::<i32>().unwrap();
    println!("x = {}", x.to_string());

    //
    // Conversion Between &str and String Types
    let text = "Hello, world!"; // text is a string slice, &str

    //
    // Convert a string slice to a String
    let s = text.to_string();
    // Convert a String to a string slice, &str
    let str_slice = s.as_str();

    // print str_slice's value
    println!("str_slice's value: {:?}", str_slice);

    println!("s = {:?}", s);
}

use std::any::type_name;

pub fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>());
}

pub fn get_type_of<T>(_: &T) -> String {
    type_name::<T>().to_string()
}

pub fn print_variable_type() {
    let x = 5;
    let y = 5.0;
    let z = 'a';
    let a = "Hello, world!";
    let b = true;

    println!("y is of type: {:?}", get_type_of(&y)); //can only print values

    print_type_of(&x);
    print_type_of(&y);
    print_type_of(&z);
    print_type_of(&a);
    print_type_of(&b);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_variables() {
        convert_variables();
    }

    #[test]
    fn test_print_variable_type() {
        print_variable_type();
    }
}