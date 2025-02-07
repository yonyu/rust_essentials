//	At any given time, you can have either one mutable reference or any number of immutable references.
//	References must always be valid.


fn multiple_immutable_references() {
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    assert_eq!(r1, r2);

    println!("The first reference: {}", r1);
    println!("The second reference: {}", r2);
}

// cannot borrow `s` as immutable because it is also borrowed as mutable
fn one_mutable_reference() {
    let mut s = String::from("hello");

    let r = &mut s; // mutable borrow occurs here

    println!("The original value: {}", s); // immutable borrow occurs here, this is not allowed

    println!("The mutable reference: {}", r); //mutable borrow used here
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiple_immutable_references() {
        multiple_immutable_references();
    }

    #[test]
    fn reference_test1() {
        let mut s1 = String::from("hello");
        let r1 = &mut s1;
        r1.push_str(", world!");
        // let l1 = s1.len();
        // let l2 = r1.len();
        println!("reference r1: {}", r1);
        //assert_eq!(s1.len(), r1.len());
        //assert_eq!(l1, l2);
    }

    // cannot borrow `s1` as mutable more than once at a time
    #[test]
    fn reference_test2() {
        let mut s1 = String::from("hello");
        let r1 = &mut s1;
        //let r2 = &mut s1;
        r1.push_str(", world!");

        println!("reference r1: {}", r1);
    }

    // cannot borrow `s1` as immutable because it is also borrowed as mutable
    #[test]
    fn reference_test3() {
        let mut s1 = String::from("hello");
        //let r1 = &mut s1;
        //let r1 = &s1;
        let r2 = &s1;
        let r3 = &s1;

        println!("original s1: {}; reference r1: {}", s1, r3);

        let r4 = &mut s1;
        r4.push_str(", world!");
        println!("reference r4: {}", r4);
    }
}