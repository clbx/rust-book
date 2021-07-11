use std::fs::File;
use std::io::Read;
use std::io; 
use std::io::ErrorKind;

fn main() {
    
    //panic!("crash and burn");


    //let v = vec![1,2,3];
    //v[99];

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => {
                    println!("Created file!");
                    fc},
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    let f = File::open("bello.txt").expect("Failed to open bello.txt");

}


fn read_username_from_file() -> Result<String, io::Error> {
    
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }

}