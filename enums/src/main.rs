mod define_enums;
mod match_with_default_placeholder;
mod enum_methods;
mod option_enum;
mod matching_option;
mod if_let_syntax;
mod challenge_represent_location;

fn main() {
    // let my_shape = define_enums::Shape::Rectangle(1.2, 3.4);
    // println!("my_shape is a {:?}", my_shape);
    //
    // match my_shape {
    //     define_enums::Shape::Circle(r) => println!("Circle with radius: {}", r),
    //     define_enums::Shape::Rectangle(w, h) => println!("Rectangle width: {}, height: {}", w, h),
    //     define_enums::Shape::Triangle(a, b, c) => println!("Triangle with sides: {}, {}, {}", a, b, c),
    // }
    //
    // let number = match_with_default_placeholder::match_with_default(2);
    // println!("number is {}", number);
    //
    // let number = match_with_default_placeholder::match_with_default(5);
    // println!("number is {}", number);
    //
    // let rect = enum_methods::Shape::Rectangle(1.0, 2.0);
    // let perimeter = rect.get_perimeter();
    // println!("rect perimeter is {}", perimeter);
    //
    // option_enum::demonstrate();
    //matching_option::demonstrate();
    //if_let_syntax::if_let_syntax();
    challenge_represent_location::exercise();
}
