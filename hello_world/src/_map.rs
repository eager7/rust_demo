use std::collections::HashMap;

#[test]
fn _map() {
    let mut scores = HashMap::new();
    scores.insert("blue".to_string(), 10);
    scores.insert("yellow".to_string(), 50);
    scores.insert("yellow".to_string(), 40);
    scores.entry("yellow".to_string()).or_insert(20);
    let n = scores.entry("yellow".to_string()).or_insert(20);
    *n += 5;
    println!("{:#?}", scores);

    let teams = vec!["blue".to_string(), "yellow".to_string()];
    let scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.into_iter().zip(scores.into_iter()).collect();
    println!("{:#?}", scores);

    println!("{:?}", scores.get(&"blue".to_string()));

    for (key, value) in scores {
        println!("{},{}", key, value);
    }
}
