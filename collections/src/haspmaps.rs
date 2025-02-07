use std::collections::HashMap;

pub fn hashmap_exercise() {
    let mut missions_flow = HashMap::new(); // missions flown as of 1 Jan 2021
    missions_flow.insert("Hadfield", 3);
    missions_flow.insert("Hurley", 3);
    missions_flow.insert("Barron", 0);
    println!("missions_flow is {:?}", missions_flow);

    // update
    missions_flow.insert("Barron", 4);
    missions_flow.entry("Barron").or_insert(2);

    let barron_missions = missions_flow.get("Barron");
    println!("Barron missions is {:?}", barron_missions);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hashmap_exercise() {
        hashmap_exercise();
    }
}