use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please imput your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() { //match matches the output of parse which is either Ok or Err 
            Ok(num) => num,     //If its okay, than the num returns
            Err(_) => continue, //Understore is catch all, just returns
        };


        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
