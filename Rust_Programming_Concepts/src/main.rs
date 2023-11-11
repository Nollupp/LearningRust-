const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// Enums in rust:
enum IpAddrKind {
    V4,
    V6,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Struct definition in rust - very similar to C
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn using_struct()
{
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

}
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn ownership()
{
    let mut s  = String::from("Hello");
    println!("{s}");
    s.push_str(" World");
    println!("{s}");

    // This is called a "move": When S2 = S1, S1 is marked no longer valid

    let mut s1 = String::from("Move");
    let s2 = s1; // S1 is no longer valid! S2 is where the string is now stored

    println!("{s2}");

    // This is called a clone: When S2 is cloned from S1, both are valid

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);    // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

    // If you want to pass a variable into a function, you can have the function return the ownership!

    // Using references to bypass giving ownership:
    let s  = String::from("Hiya");
    let strlen = calculate_length(&s);
    println!("String length: {strlen}");

}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn scope()
{
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn constants()
{
    println!("Constant is : {THREE_HOURS_IN_SECONDS}");
}

fn main() {
    //scope();
    //constants();
    //ownership();
    using_struct();
}
