// must use, cause HashMap not include prelude.
use std::collections::HashMap;

fn main() {

    let mut score = HashMap::new();

    score.insert(String::from("team1"), 89);
    score.insert(String::from("team2"), 99);

    let x = score.get(&String::from("team1"));

    for (k,v) in &score {
        println!("{} {}", k, v);
    }

    let teams = vec![String::from("team3"), String::from("team4")];
    let scores = vec![32,43];

    let score_map: HashMap<_,_> = teams.iter().zip(scores.iter()).collect();

    let a = String::from("123");
    let b = String::from("345");
    let mut map = HashMap::new();
    map.insert(a, b); // this will move a&b

    let mut hash_map = HashMap::new();
    hash_map.insert("1", 1);
    hash_map.insert("1", 4); // this will replace

    let mut hash_map = HashMap::new();
    hash_map.entry("1").or_insert(3); // this will insert when key "1", not point to a value.

    println!("{:?}", hash_map);

    // use hashmap to count world
    let text = "work world fuck you you fuck work fuck ass you mother";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {

        // 沒出現過就 insert 0, 出現過就 +1
        let count = map.entry(word).or_insert(0);

        *count += 1;
    }

    println!("{:?}", map);

    vector_practice();
    println!("{}", pig_latin(String::from("girst")));
    println!("{}", pig_latin(String::from("apple")));
    println!("{}", pig_latin(String::from("fuck")));
}

fn vector_practice() {
    let mut numbers: Vec<i32> = vec![332,43,43590,43,43,6,1,43,9,-4,-43,434,-9132,34,0,432];

    let mut x: f64 = 0.0;
    let mut map: HashMap<i32, i32> = HashMap::new();

    for i in &numbers {

        let count = map.entry(*i).or_insert(0);
        *count += 1;

        x += *i as f64;
    }
    
    let avg: f64 = x / (numbers.len() as f64);

    println!("avg:{}", avg);

    println!("{:?}", map);

    numbers.sort_by(|a, b| {
        a.cmp(b)
    });

    let mid: f64 = (numbers[8] + numbers[9]) as f64 / 2.0;

    println!("mid:{}", mid);
}

fn pig_latin(target: String) -> String {
    let mut result = String::new();
    let mut chars = target.chars();

    // nth() will move out iter
    let first_char = &chars.nth(0);

    if let Some(c) = first_char {
        if *c == 'a' || *c == 'e' || *c == 'i' || *c == 'o' || *c == 'u' {
            result = target + "-hay";

            return result;
        }
    }

    for c in target.chars() {
        if c != 'a' && c != 'e' && c != 'i' && c != 'o' && c != 'u' {
            result = target.replace(c, "") + "-" + c.to_string().as_str() + "ay";
            break;
        }
    }

    result
}