use std::collections::HashMap;
fn main() {
    let v: Vec<i32> = Vec ::new();
    let mut hashmap: HashMap<String, String> = HashMap ::new();
    hashmap.insert("hehe".to_string(), "da".to_string());
    hashmap.insert("haha".to_string(), "xixi".to_string());
    println!("value {:?}", hashmap);
    println!("vec is {:?}", v);
}