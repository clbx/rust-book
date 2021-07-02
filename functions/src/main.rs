fn main() {
    println!("Hello, world!");
    another_function(5,6);


    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y(1) is: {}", y);

    let x = five();
    println!("The value of x(1) is: {}", x);


    let a = [10,20,30,40,50];
    let mut index = 0;

   for element in a.iter(){
       println!("the value is: {}", element);
   }
}

fn another_function(x: i32, y: i32){
    println!("The value of x is : {}",x);
    println!("The value of y is : {}",y);


}


fn five() -> i32 {
    5
}

