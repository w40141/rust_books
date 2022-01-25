use std::collections::HashMap;

pub fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Red"), String::from("Green")];
    let initial_scores = vec![20, 30];

    let scores1: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let field_name = String::from("favorite color");
    let field_value = String::from("blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    let score1 = scores.get("Yellow");

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Blue"), 10);
    scores2.insert(String::from("Blue"), 25);
    scores2.entry(String::from("Yellow")).or_insert(50);
    scores2.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores2);

    let text = "hello world wonderful world";
    let mut map1 = HashMap::new();

    for word in text.split_whitespace() {
        let count = map1.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map1);
}
