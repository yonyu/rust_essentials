pub fn demonstrate() {
    let nothing: Option<i32> = None;
    let v1 = Some(1);

    let countdown = [5, 4, 3, 2, 1];
    //let number = countdown[5];
    //let number = countdown.get(5);
    let number = countdown.get(4);
    println!("number is {:?}", number);
    let number = number.unwrap_or(&0) + 1;
    println!("number is {}", number);
}

