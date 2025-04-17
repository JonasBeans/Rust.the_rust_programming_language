#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

#[derive(Debug)]
struct Square {
    width: u32,
    height: u32
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Square {
    fn new(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}


#[derive(Debug)]
struct User {
    name: String,
    last_name: String,
    age: u32
}

fn main() {
    user_example();
    rectangle_example();
    square_example();
}

fn square_example() {
    let square = Square::new(30);
    println!("I'm totally {:?} with you!", square);
}

fn rectangle_example() {
    let rect1 = Rectangle::new(10, 20);
    let rect2 = Rectangle::new(5, 7);
    let rect3 = Rectangle::new(20, 30);

    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("The area of the rectangle is {} square pixels.", rect2.area());
    println!("The area of the rectangle is {} square pixels.", rect3.area());

    println!("Rectangle 2 can fit in Rectangle 1: {}", rect1.can_hold(&rect2));
    println!("Rectangle 3 can fit in Rectangle 1: {}", rect1.can_hold(&rect3));
}

fn user_example(){
    let user = User {
        name: String::from("Jonas"),
        last_name: String::from("Boon"),
        age: 24
    };

    println!("Hello, {} {}!", user.name, user.last_name);
    println!("Full struct {user:?}");
    println!("Full struct formatted {user:#?}");
    dbg!(user.age); // stderr example
}
