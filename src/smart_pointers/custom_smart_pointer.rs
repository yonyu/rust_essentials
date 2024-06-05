// Implementing Drop trait

//use std::ops::Drop; // it is in prelude by default

#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop() {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };

        let _d = CustomSmartPointer {
            data: String::from("other stuff"),
        };

        // drop a value earlier by calling std::mem::drop
        drop(c);

        println!("Smart Pointers created.");
    }
}