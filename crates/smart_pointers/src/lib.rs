pub mod mybox;
pub mod recursive_type_with_box;
pub mod custom_smart_pointer;
pub mod reference_counting;
pub mod limited_tracker;
pub mod multiple_owners_of_mutable_data;
pub mod reference_cycles;
pub mod weak_references;
pub mod interior_mutability;


//use smart_pointers::recursive_type_with_box::List::{Cons, Nil};

fn main() {
    //smart_pointers::recursive_type_with_box::instantiate_list();

    reference_cycles::demo_cycles();
}
