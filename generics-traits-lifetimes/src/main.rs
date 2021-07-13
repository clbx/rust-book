use crate::lib::Summary;

mod lib;

fn main() {
    

    /* 
    let number_list = [34,50,25,100,65];

    let result = largest(&number_list);

    println!("The largest number is {}", result);

    let char_list = vec!['y','m','a','q'];

    let result = largest(&char_list);

    println!("The largest character is {}", result);

    let point = Point{ x:5, y:5.0};
    */

    let tweet = lib::Tweet{
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply:false,
        retweet: false,
    };



    println!("One new tweet: {}",tweet.summarize());

}

/** 
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
*/

struct Point<T,U> {
    x: T,
    y: U,
}

