fn main() {
    
    let mut s = String::from("Hello");

    s.push_str(", world!");

    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);


    let s = String::from("Hello");

    takes_ownership(s);

    println!("{}", gives_ownership());
    
    println!("{}", takes_and_gives_back(String::from("hello")));

    let s1 = String::from("Hello");
    let (s2,len) = calculate_length(s1);

    println!("The length of {} is {}",s2,len);

    //References

    let s1 = String::from("References");

    let len = calculate_length_but_with_references(&s1);

    println!("The length of {} is {}",s1,len);

    let mut changeme = String::from("this is the");

    change(&mut changeme);

    println!("{}",changeme);

    let s = String::from("hello");
    let slice = &s[..2];

    println!("{}", slice);

}

fn change(some_string: &mut String) {

    some_string.push_str(" world");

}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}


fn gives_ownership() -> String{
    let some_string = String::from("Hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize){
    let length = s.len();

    (s,length)
}

fn calculate_length_but_with_references(s: &String) -> usize {
    s.len()
}