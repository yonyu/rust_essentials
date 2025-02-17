pub mod garden;
pub mod foo;
pub mod bar;

#[warn(unused_imports)]
use crate::garden::vegetables::Asparagus;
use crate::foo::foo;
//use crate::bar::say_foo;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);

    foo();
    bar::say_foo();
}
