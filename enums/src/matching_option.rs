pub fn demonstrate() {
    let countdown: [i32; 5] = [5, 4, 3, 2, 1];
    //let number = countdown[5];
    let number = countdown.get(2);
    let number = match(number) {
        Some(n) => &n,
        None => &0
    };
    println!("number is {}", number);
}