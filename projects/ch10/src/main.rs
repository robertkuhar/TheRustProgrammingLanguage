// There has to be a Trait or something to say "I want just Numbers here"
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Mixed_Type_Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Mixed_Type_Point<T, U> {
    fn mixup<V, W>(self, other: Mixed_Type_Point<V, W>) -> Mixed_Type_Point<T, W> {
        Mixed_Type_Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let first_list = vec![34, 50, 25, 100, 65];
    println!(
        "The largest number in {:?} is {}",
        first_list,
        largest_i32(&first_list));
    let second_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    println!(
        "The largest number in {:?} is {}",
        second_list,
        largest_i32(&second_list));

    let third_list = vec!['a', 'z', 'm', '0', '.'];
    println!(
        "The largest char in {:?} is {}",
        third_list,
        largest_char(&third_list));
    println!(
        "The largest char in {:?} is {}",
        third_list,
        largest(&third_list));

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p1 = Mixed_Type_Point { x: 5, y: 10.4 };
    let p2 = Mixed_Type_Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    return largest;
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
