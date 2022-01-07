use std::io;
use std::collections::HashMap;
use rand::Rng;
fn main() {
    let mut user1 = User {
        email: String::from("1@gmail.com"),
        username: String::from("sok"),
        active:true,
        sign_in_count: 1
    };
    user1.email = String::from("another@gmail.comn");
    println!("{}",user1.email);
    
    let email = String::from("tt@gmail.com");
    let username = String::from("testddd");
    let user2 = build_user(email,username);
    let user3 = User{
        email: String::from("sdfsdf"),
        ..user1
    };

    let rect1 = Rect {
        width: 30,
        height: 50,
    };
    let rect2 = Rect {
        width: 10,
        height: 40,
    };
    let rect3 = Rect {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2?{}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect2?{}", rect1.can_hold(&rect3));
    
    let four = IpAddKind::V4(String::from("sdfsdf"));
    let six = IpAddKind::V6;
    four.call();
    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other)

    }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("{}",max);
    }

    let secrete_number = rand::thread_rng().gen_range(1..101);

}

// fn samples(){
// let mut x = 5;
//     println!("The value of x is {}",x);
//     x = 6;
//     println!("The value of x is {}",x);
//     const THREE_HOURS_SECONDS: u32 = 60 * 60 * 3;
//     print!("{}",THREE_HOURS_SECONDS);
//     let guess: u32 = "42".parse().expect("Not a number");
//     print!("{}",guess);
    
//     //Tuple 
//     let tup = (500, 6.4, 1);
//     let (x,y,z) = tup;
//     println!("The value of y is : {}",y);
//     print!("first number:{}",tup.0);

//     //Array
//     let months = ["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","OCT","Nov","Dec"];
//     for elem in months {
//         print!("\n{}",elem);
//     }
    
//     let a:[i32;5] = [1,2,3,4,5];
//     for elem in a {
//         println!(" {}",elem);
//     }

//     let b = [3;5];
//     for elem in b {
//         println!(" {}",elem);
//     }

//     let a =[1,2,3,4,5];

//     println!("please enter an array index.");
//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");
    
//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");

//     let element = a[index];
//     println!(
//         "The value of the element at index {} is : {}",
//         index,element
//     )
// }

// fn print_labeled_measurement(value: i32, unit_label: char){
//     println!("The measurement is:{}{}",value,unit_label);
// }

// //statement
// fn testStatement_expressions(){
//     let y = {
//         let x = 3;
//         x+1
//     };
//     println!("expression has return value:{}",y);
// }

// //function with return value
// fn getFive() -> i32 {
//     return 5
// }

// //test control flow 
// fn test_control_flow(){
//      let mut count = 0;
//     'counting_up: loop {
//         println!("count = {}", count);
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {}", remaining);
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {}", count);           
// }

// fn conditionLoop() {
//     let mut number = 3;
//     while number != 0 {
//         println!("{}!", number );
//         number -= 1
//     }
//     println!("LIFEOFF!");
// }

// //OwnerShip 
// fn testOwnerShip(some_string: &mut String){
//     some_string.push_str("world");
//     println!("{}",some_string);
// }

// fn make_copy(some_integer: i32){
//     println!("{}",some_integer);
// }

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i..s.len()].trim();
        }
    }
    return &s[..];
}

fn build_user(email: String, username: String) -> User {
    User{
        email: email,
        username: username,
        active: true,
        sign_in_count: 1
    }
}

struct User {
    active:bool,
    username:String,
    email:String,
    sign_in_count: u64
}

struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

struct Rect {
    width: u32,
    height: u32
}

impl Rect {
    fn area(&self) -> u32 {
        self.width*self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[derive(Debug)]
enum IpAddKind {
    V4(String),
    V6(String)
}

impl IpAddKind {
    fn call(&self) {
        println!("testing enum");
    }
}

#[derive(Debug)]
enum  UsState {
    Alambama,
    Alaska,
}

enum Coin {
    Peny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8  {
    match coin {
        Coin::Peny => {
            println!("Lucky penny!");
            return 1;
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quater from {:?}!", state);
            return 25
        },
    }
}

fn plus_one(x:Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn add_fancy_hat() {

}

fn remove_fancy_hat() {

}

fn move_player (num_space : u8) {}