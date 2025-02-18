mod borrow_checker;
mod lifetime_annotation;
mod struct_lifetime_annotations;

fn main() {
    borrow_checker::demonstrate_borrowed_value_lifetime();
    lifetime_annotation::demonstrate_lifetime_annotation();

    let vehicle =  struct_lifetime_annotations::Shuttle {
        name: "Endeavour",
    };

    let message = String::from("Ground Control");
    let sender = vehicle.send_transmission(&message);
    println!("sender is {}", sender);
}
