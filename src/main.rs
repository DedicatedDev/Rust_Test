use std::io;
use std::collections::HashMap;
use rand::Rng;
use std::fs::File;
use std::io::ErrorKind;

mod models;
mod traits;

use models::new_article::NewArticle;
use models::tweet::Tweet;
use traits::summary::Summary;


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
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    
    let third: &i32 = &v[2];
    println!("The third element is {}",third);
    
    let data = "initial contents";
    let s = data.to_string();

    let s1 = String::from("Hello,");
    let s2 = String::from("world!");
    let s3 = String::from("dkdkd");


    let ss = format!("{}-{}-{}",s1,s2,s3);
    println!("{}",&ss[0..1]);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}",scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };


    //let f = File::open("hello.txt");

    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     },
    // };
    
    // let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];

    //let result = largest(&char_list);
    //println!("The largest char is {}", result);



}

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     return largest;
// }



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

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

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
