pub enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64) // with latitude and longitude stored as float values
}

impl Location {
    pub fn display(&self) {
        match self {
            Location::Unknown => println!("Unknown Location"),
            Location::Anonymous => println!("Anonymous Location"),
            Location::Known(lat, lon) => println!("Latitude: {}, Longitude: {}", lat, lon)
        }
    }
}

pub fn exercise() {
    let location = Location::Unknown;
    location.display();

    let location = Location::Anonymous;
    location.display();

    let location = Location::Known(37.7749, -122.4194);
    location.display();
}