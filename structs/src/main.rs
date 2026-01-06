#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

// implementation block: functions and methods associated with a struct
impl Rectangle{
    // associated method - tied to an instance of a struct 
    // they have &self as an argument
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // associated method - tied to an instance of a struct
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

// struct allow multiple impl blocks
impl Rectangle{
    fn square(size: u32)-> Rectangle {
        Rectangle{
            width: size,
            height: size,
        }
    }
}

fn main(){
    // find the area of a rectangle
    // let width1 = 30;
    // let height1 = 50;

    let rect = Rectangle{
        width: 30,
        height: 50,
    };

    let rect1 = Rectangle{
        width: 20,
        height: 40,
    };

    let rect2 = Rectangle{
        width: 40,
        height: 50,
    };

    let rect3 = Rectangle::square(25);

    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));

    println!("rect: {:#?}", rect);

    println!("The area of the rectangle is {} square pixels.",
        rect.area()
    );
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// ----------------------------------------------------------
// struct User{
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool, 
// }

// fn main() {
    
//     let mut user1 = User{
//         email: String::from("user1@gmail.com"),
//         username: String::from("user1"),
//         active: true,
//         sign_in_count: 1,
//     };
//     let name = user1.username;
//     user1.username = String::from("newuser");

//     let user2 = build_user(
//         String::from("user2@gmail.com"), 
//         String::from("user2")
//     );

//     let user3 = User{
//         email: String::from("james@gmail.com"),
//         username: String::from("james"),
//         ..user2
//     };

//     // ----------------
//     //tuple structs: when you want the entire tuple to have a name and be of different type than other tuple
//     struct Color(i32, i32, i32);
//     struct Point(i32, i32, i32);

// }

// fn build_user(email: String, username: String) -> User{
//     User{
//         email,
//         username,
//         active: true,
//         sign_in_count: 1
//     }
// }