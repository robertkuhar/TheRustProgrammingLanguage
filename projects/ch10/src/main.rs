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
}

fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest= list[0];
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
