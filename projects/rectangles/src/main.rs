use std::fmt;

fn main() {
    // Listing 5-8 is a 2 argument function
    let width1 = 30;
    let height1 = 50;
    println!(
        "area_5_8(width: {}, height: {}) => {}",
        width1,
        height1,
        area_5_8(width1, height1)
    );

    // Listing 5-9 leverages tuples in the function call
    // Note the funky {:?} to get the tuple to println!
    // https://stackoverflow.com/questions/38157335/what-does-mean-in-a-rust-format-string
    let rect1 = (30, 50);
    println!(
        "area_5_9(dimensions: {:?}) => {}",
        rect1,
        area_5_9(rect1)
    );

    // Listing 5-10 leverages a struct
    // We impl fmt::Display for the struct to allow println! to work
    let rect2 = Rectangle { height: 50, width: 30 };
    println!(
        "area_5_10(rectangle: {}) => {}",
        rect2,
        area_5_10(&rect2)
    );

    let rect3 = Rect { h: rect2.height, w: rect2.width };
    println!(
        "area_5_10a(rect: {:?}) => {}",
        rect3,
        area_5_10a(&rect3)
    );

    // both of the following println! required the derive(Debug) thing
    let one_two_three = XYZ { x: 1, y: 2, z: 3 };
    println!("one_two_three: {{:?}} is compact {:?}", one_two_three);
    println!("one_two_three: {{:#?}} is some sort of pretty print {:#?}", one_two_three);

    // Listing 5-13 adds a area method to the Rectangle struct
    println!("Rectangle.area_5_13(): {}", rect2.area_5_13());

    // Listing 5-15 adds a 2nd method to Rectangle
    let rect_30x50 = Rectangle { width: 30, height: 50 };
    let rect_10x40 = Rectangle { width: 10, height: 40 };
    let rect_60x45 = Rectangle { width: 60, height: 45 };
    println!(
        "Can {} hold {}? {}",
        rect_30x50,
        rect_10x40,
        rect_30x50.can_hold(&rect_10x40));
    println!(
        "Can {} hold {}? {}",
        rect_30x50,
        rect_60x45,
        rect_30x50.can_hold(&rect_60x45));

    println!("square is {}", Rectangle::make_square(10));
}

fn area_5_8(width: u32, height: u32) -> u32 {
    return width * height;
}

fn area_5_9(dimensions: (u32, u32)) -> u32 {
    return dimensions.0 * dimensions.1;
}

fn area_5_10(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}

fn area_5_10a(rect: &Rect) -> u32 {
    return rect.w * rect.h;
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area_5_13(&self) -> u32 {
        return self.height * self.width;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }

    fn make_square(size: u32) -> Rectangle {
        return Rectangle { width: size, height: size };
    }
}

// This is some toString concept.
// https://stackoverflow.com/questions/27769681/should-i-implement-display-or-tostring-to-render-a-type-as-a-string
// https://doc.rust-lang.org/std/fmt/trait.Display.html
impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(width: {}, height: {})", self.width, self.height)
    }
}

#[derive(Debug)]
struct Rect {
    w: u32,
    h: u32,
}

#[derive(Debug)]
struct XYZ {
    x: u32,
    y: u32,
    z: u32,
}

