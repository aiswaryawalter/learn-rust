struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool, 
}

fn main() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    
    let mut user1 = User{
        email: String::from("user1@gmail.com"),
        username: String::from("user1"),
        active: true,
        sign_in_count: 1,
    };
    let name = user1.username;
    user1.username = String::from("newuser");

    let user2 = build_user(
        String::from("user2@gmail.com"), 
        String::from("user2")
    );

    let user3 = User{
        email: String::from("james@gmail.com"),
        username: String::from("james"),
        ..user2
    };



}

fn build_user(email: String, username: String) -> User{
    User{
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}