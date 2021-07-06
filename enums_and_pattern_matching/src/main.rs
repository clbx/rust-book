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
    


}
