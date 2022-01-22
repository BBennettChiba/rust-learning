use std::collections::HashMap;

pub fn run (){
    let blue = String::from("blue");
    let green = String::from("green");
    let mut hash_map = HashMap::new();

    hash_map.insert(blue, 10);
    hash_map.insert(green, 20);

    let key = String::from("blue");
    let score = hash_map.get(&key);
    match score {
        Some(score) => println!("{}", score),
        None => println!("None"),
    }

    for (key, value) in hash_map{ 
        println!("{}: {}", key, value);
    }

    let mut hash_map2 = HashMap::new();
    hash_map2.insert(String::from("purple"), 10);
    hash_map2.insert(String::from("purple"), 20);

    hash_map2.entry(String::from("yellow")).or_insert(30);
    hash_map2.entry(String::from("yellow")).or_insert(50);

    println!("{:?}", hash_map2);

    let text = "Hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}