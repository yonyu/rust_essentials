struct Sheep {}

struct Cow {}

pub trait Animal {
    fn noise(&self) -> &'static str;
}

impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

fn random_animal(random_number: f64) -> Box<dyn Animal> {
    println!("random_number: {}", random_number);
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

#[cfg(test)]
mod tests {
    use rand::prelude::*; // https://rust-random.github.io/rand/rand/fn.rng.html
    use super::*;

    #[test]
    fn test_random_animal() {
        let mut rng = rand::rng();
        let random_number = rng.random::<f64>();
        let animal = random_animal(random_number);
        println!("You have randomly choosen an animal, and it says {}", animal.noise());
    }
}
