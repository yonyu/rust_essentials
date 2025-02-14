// Specify constraint trait: T: std::ops::Add<Output= T>
fn sum_boxes<T: std::ops::Add<Output= T>>(box1: Box<T>, box2: Box<T>) -> Box<T> {
    Box::new(*box1 + *box2)
}

#[cfg(test)]
mod tests {
    use std::mem;
    use super::*;
    #[test]
    fn test_sum_boxes() {
        let box1: Box<i32> = Box::new(1);
        let box2: Box<i32> = Box::new(2);
        let box3 = sum_boxes(box1, box2);
        assert_eq!(*box3, 3);

        let pi = Box::new(3.14159);
        let e = Box::new(2.71828);
        assert_eq!(*sum_boxes(pi, e),  5.85987);

        println!("box 3 is {} bytes", mem::size_of_val(&box3));
    }
}
