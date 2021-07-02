use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is : {}", x);
    x = 6; 
    println!("The value of x is : {}",x);


    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    let spaces = "     ";
    let spaces = spaces.len();

    /*
    *
    * let mut spaces = "    "; 
    * spaces = spaces.len(); doesnt work because its mutable and can't change 
    *
    */

    let x = 2.0;

    let y: f32 = 3.0;

    let sum = 5 + 10;
    let difference = 95.5 - 2.4;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43%5;

    let t = true; 
    let f: bool = false; 


    let a = [1,2,3,4,5]; 

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
