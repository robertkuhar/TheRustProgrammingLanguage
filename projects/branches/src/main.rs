fn main() {
    let number = 3;
    less_than_bound(number, 5);
    less_than_bound(5, 5);
    less_than_bound(7, 5);

    divisible_by(5);
    divisible_by(4);
    divisible_by(9);
    divisible_by(2);
    divisible_by(1);
    divisible_by(0);

    loop_n_times(3);

    countdown_with_while(3);
    for_loop(&[1, 2, 4]);
    better_for_loop(&[10, 9, 8]);
}

fn divisible_by(number: i32) {
    if number % 4 == 0 {
        println!("{} is divisible by 4", number);
    } else if number % 3 == 0 {
        println!("{} is divisible by 3", number);
    } else if number % 2 == 0 {
        println!("{} is divisible by 2", number);
    } else {
        println!("{} is not divisible by 4, 3, or 2", number);
    }
}

fn less_than_bound(number: i32, bound: i32) {
    if number < bound {
        println!("{} < {} is true", number, bound);
    } else {
        println!("{} < {} is false", number, bound);
    }
}

fn loop_n_times(times: i32) {
    let mut i = 0;
    loop {
        i = i + 1;
        if i > times {
            break;
        }
        println!("again!");
    }
}

fn countdown_with_while(times: i32) {
    let mut number = times;
    // I can't figure out how to get while to return a value
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn for_loop(inputs: &[i32]) {
    let mut index: i32 = 0;
    // I can't figure out how to get for to return a value
    for input in inputs.iter() {
        println!("the value at index {} is {}", index, input);
        index += 1;
    }
}

fn better_for_loop(inputs: &[i32]) {
    for (i, item) in inputs.iter().enumerate() {
        println!("The {}th item is {}", i + 1, item);
    }
}
