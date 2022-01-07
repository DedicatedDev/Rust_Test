use std::io;
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