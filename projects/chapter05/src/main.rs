use std::fmt;

fn main() {
    struct_stuff();
    tuple_structs();
    do_area_stuff();
}

fn do_area_stuff() {
    let width = 2;
    let height = 3;
    println!("area( width: {}, height: {}):\t{}", width, height, area(width, height));
    let dimensions = (width, height);
    // bizarre.  What does the :? do in there?
    // https://stackoverflow.com/questions/38157335/what-does-mean-in-a-rust-format-string
    println!("area_of_tuple( dimensions: {:?}):\t{}", dimensions, area_of_tuple(dimensions));
    let rectangle_2x4 = Rectangle { width: 2, height: 4 };
    let rectangle_2x3 = Rectangle { width, height };
    println!("area_of_Rectangle( {}):\t{}", rectangle_2x4, area_of_Rectangle(&rectangle_2x4));
    println!("area_of_Rectangle( {}):\t{}", rectangle_2x3, area_of_Rectangle(&rectangle_2x3));
}

fn area(width: u32, height: u32) -> u32 {
    return width * height;
}

fn area_of_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_of_Rectangle(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

struct Rectangle {
    height: u32,
    width: u32,
}

// This is some toString concept.
// https://stackoverflow.com/questions/27769681/should-i-implement-display-or-tostring-to-render-a-type-as-a-string
// https://doc.rust-lang.org/std/fmt/trait.Display.html
impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(width: {}, height: {})", self.width, self.height)
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn struct_stuff() {
    let rkuhar = User {
        username: String::from("rkuhar"),
        email: String::from("notrkuhar@example.com"),
        sign_in_count: 0,
        active: true,
    };

    // error[E0277]: `User` doesn't implement `std::fmt::Display`
    // println!("rkuhar: {}", rkuhar);
    println!("rkuhar.email: {}", rkuhar.email);

    let mut mutable_rkuhar = rkuhar;
    mutable_rkuhar.sign_in_count = 1;
    println!("mutable_rkuhar.sign_in_count: {}", mutable_rkuhar.sign_in_count);

    let ckuhar = build_user(String::from("notckuhar@example.com"), String::from("ckuhar"));

    let cckuhar = build_user_field_init_shorthand(
        String::from("notckuhar@example.com"),
        String::from("cckuhar"));

    let ccckuhar = User {
        email: String::from("notckuhar@example.com"),
        username: String::from("ccckuhar"),
        ..cckuhar
    };
    println!("ckuhar.username: {}", ckuhar.username);
    println!("cckuhar.username: {}", cckuhar.username);
    println!("ccckuhar.username: {}", ccckuhar.username);
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user_field_init_shorthand(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 0,
    }
}

fn tuple_structs() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
