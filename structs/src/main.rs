struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32,i32,i32);
struct Point(i32,i32,i32);


fn build_user(email: String, username: String) -> User {
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}


fn main() {
    
    let user1 = User{
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = build_user(String::from("someoneelse@example.com"),String::from("someoneeleseusername123"));

    let user3 = User{
        email: String::from("anotheruser@example.com"),
        username: String::from("anotheruser123"),
        ..user1
    };

    println!("1 {} and 2 {}",user1.username, user2.username);

    let black = Color(0,0,0);
    let center = Point(0,0,0);



}

