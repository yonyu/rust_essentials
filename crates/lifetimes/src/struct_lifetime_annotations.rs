/*
// the name has string data type
pub struct Shuttle {

    pub name: String,
}

impl Shuttle {
    pub fn send_transmission(&self, message: &str) -> &str{
        println!("Transmitting message: {}", message);

        &self.name
    }
}*/

pub struct Shuttle<'a> {
    pub name: &'a str,
}

// impl<'a> Shuttle<'a> {
//     pub fn send_transmission(&self, message: &str) -> &str{ // rule #3
//         println!("Transmitting message: {}", message);
//
//         self.name
//     }
// }

impl<'a, 'b> Shuttle<'a> {
    pub fn send_transmission(&'a self, message: &'b str) -> &'b str{
        println!("Transmitting message: {}", message);

        message
    }
}