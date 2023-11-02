// enum IpAddressKind {
//   V4(u8,u8,u8,u8),
//   V6(u8,u8,u8,u8)
// }

// struct IpAddress {
//   kind: IpAddressKind,
//   address: String
// }

// enum Message {
//   Quit,
//   Move {x: i32, y: i32},
//   Write(String),
//   ChangeColor(i32,i32,i32)
// }

// impl Message {
//   fn some_function() {
//     print!("This is some function")
//   }
// }

// fn main() {
//   println!("Hello, world!");

//   let four = IpAddressKind::V4;
//   let six = IpAddressKind::V6;

//   let localhost = IpAddressKind::V4(127,0,0,1);
// }
// fn route(ip_kind: IpAddressKind) {}

///// The Option Enum
#[derive(Debug)]
enum UsState {
  Akabama,
  Alaska,
  Arizona,
  Arkansas,
  California,
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
      println!("State quarter from {:?}!", state);
      25
    },
  }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
    _ => None
  }
}

fn main() {
  // enum Option<T> {
  //   Some(T),
  //   None,
  // }

  // let some_number = Some(5);
  // let some_string = Some("a string");

  // let absent_number: Option<i32> = None;

  // let x = 5;
  // let y: Option<i8>  = None;

  // let sum = x + y.unwrap_or(0);

  // value_in_cents(Coin::Quarter(UsState::California));

  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);

}