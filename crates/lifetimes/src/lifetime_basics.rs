

/*
pub fn lifetime_and_scope() {

    let reference;
    {
        let value = 10; // value's lifetime is within this block
        reference = &value;
    }

    // outside the block, value is out of scope
    println!("Value at reference: {}", reference);
}
*/

fn foo(x: &i32) -> &i32 {
    x
}

fn foo2<'a>(x: &'a i32, y: &i32) -> &'a i32 {
    x
}