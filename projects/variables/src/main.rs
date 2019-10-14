fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let i = 1;
    {
        // Interesting.  Note that the inference here is for a pointer to str, not String?!?
        let i = "jello";
        println!("the value of j here is: {}", i);
    }
    println!("the value of j here is {}", i);

    // Tuples get the type inference thing.
    let tup = (1.0, 1, 2, 3.14, true);
    // You destructure to get the stuff back out.
    let (f, _, _, _, b) = tup;
    println!("The value of f is {}.  The value of b is {}.", f, b);
    println!("at index 3 we should get PI: {}", tup.3);
    // Note that going for tup.10 is a compile time error

    let a0 = [1, 2, 3, 4, 5];
    let a1 = [3; 5];
    // I don't know exactly what that monkey thing {:?} is.
    println!("a0 is {:?}", a0);
    println!("a1 is {:?}", a1);
    another_function();
}

fn another_function() {
    println!("Another function.");
}
