#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    // update value
    scores.insert(String::from("blue"), 9000000);

    // Only Inserting a Value If the Key Has No Value
    scores.entry(String::from("yellow")).or_insert(800);
    scores.entry(String::from("red")).or_insert(777);


    let key_blue = String::from("blue");
    let score_val = scores.get(&key_blue);

    println!("scores = {:?}, blue = {:?}", scores, score_val);

    for (k, v) in &scores {
        println!("{}, {}", k, v);
    }

    let teams = vec![String::from("blue"), String::from("yellow")];
    let initial_scores = vec![10, 50];
    let mut scores_2: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    // Hash Maps and Ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and see what compiler error you get!


    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";
    let mut map_txt = HashMap::new();
    for w in text.split_whitespace() {
        let count = map_txt.entry(w).or_insert(0);
        *count += 1;
    }
    println!("map_txt = {:?}", map_txt);
}
