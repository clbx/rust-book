use std::collections::HashMap;

fn main() {
    
    let v: Vec<i32> = Vec::new();

    let mut v = vec![1,2,3];

    v.push(5);
    v.push(6);
    v.push(7);

    let third: &i32 = &v[2];

    println!("The third element is {}", third);

    match v.get(2){
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element!"),
    }

    for i in &v {
        println!("{}",i);
    }


    let data = "initial contents";

    let s = data.to_string();
    //or let s = "initial contents".to_string();
    //or let s = String::from("initial contents");

    let mut s1 = String::from("foo");
    let s2 = "bar";

    s1.push_str(s2);

    println!("s2 is {}", s2);

    let s3 = String::from("Hello");
    let s4 = String::from("World!");

    let s5 = s3 + " " + &s4; //s1 is now no longer valid.
    println!("S5: {}",s5);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}",s1,s2,s3);

    println!("S: {}",s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yelloq"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10,50];

    let mut scores: HashMap<_,_> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("{:?}",scores);

    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();

    map.insert(field_name, field_value);
    //name and value are no longer valid


    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    //Score will be of Some(&10), will return Some or None

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }


    


}
