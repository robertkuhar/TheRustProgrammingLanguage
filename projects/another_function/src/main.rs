fn main() {
    println!("Hello, world!");
    another_function(5);
    println!("five(): {}", five());
    println!("six(): {}", six());
}

fn another_function(x: i32) {
    println!("another_function({}).", x);
}

fn five() -> i32 {
    5
}

fn six() -> i32 {
    if true { return 6; }
    /*
     * I think multi-line comments like this work too.
     */
    66
}
