fn main() {
    /*
    enum IpAddrKind{
        V4,
        V6,
    }

    struct IpAddr{
        kind: IpAddrKind,
        address: String,
    }
    
    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1");
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    fn route(ip_kind: IpAddrKind){}

    */

    enum IpAddr {
        V4(u8,u8,u8,u8),
        V6(String),
    }

    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));

    enum Message{
        Quit,
        Move {x:i32, y:i32},
        Write(String),
        ChangeColor(i32,i32,i32),
    }


    struct QuitMessate;
    struct MoveMessage{
        x: i32,
        y: i32,
    }
    struct WriteMessage(String);
    struct ChangeColorMessage(i32,i32,i32);

    impl Message {
        fn call(&self){
            //method body here
        }
    }

    let m = Message::Write(String::from("Hello"));
    m.call();
    
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky Penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state)
            },
        }
    }

    


}
