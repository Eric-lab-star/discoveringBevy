use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let mut map = HashMap::new();
    map.insert("1", "hello");
    map.insert("2", "hi");
    map.insert("2", "hello");
    for (key, val) in map.iter() {
        println!("{}: {}",key, val);
    }
}
