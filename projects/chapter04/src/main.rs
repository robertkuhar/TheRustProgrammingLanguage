fn main() {
    listing_4_2();
    listing_4_3();
    listing_4_5();
    reference_and_borrowing();
    look_ma_mutable_reference();
    slice_stuff();
}

fn listing_4_2() {
    println!("begin listing_4_2");
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s); // s's value moves into the function...
    // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
    // but i32 is Copy, so it’s okay to still use x afterward
    println!("  end listing_4_2");
}
// Here, x goes out of scope, then s. But because s's value was moved, nothing
// special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
}
// Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
}
// Here, some_integer goes out of scope. Nothing special happens.

fn listing_4_3() {
    println!("begin listing_4_3");
    let s1 = gives_ownership(); // give_ownership moves its return value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also
    // moves its return value into s3

    println!("  end listing_4_3");
}
// Here, s3 goes out of scope and is dropped.
// s2 goes out of scope but was moved, so nothing happens.
// s1 goes out of scope and is dropped.

fn gives_ownership() -> String { // gives_ownership will move its return value into the caller
    let some_string = String::from("hello"); // some_string comes into scope
    some_string // some_string is returned and moves out to the caller
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string  // a_string is returned and moves out to the calling function
}

fn listing_4_5() {
    println!("begin listing_4_5");
    let s1 = String::from("hello");
    let (s2, len) = calculate_length_of_move(s1);
    println!("The length of '{}' is {}.", s2, len);
    println!("  end listing_4_5");
}

fn calculate_length_of_move(s: String) -> (String, usize) {
    let len = s.len();
    // return (s, len);
    (s, len)
}

fn reference_and_borrowing() {
    println!("begin reference_and_borrowing");
    let s1 = String::from("hello");
    let len = calculate_length_of_reference(&s1);
    println!("The length of '{}' is {}.", s1, len);
    println!("  end reference_and_borrowing");
}

fn calculate_length_of_reference(s: &String) -> usize { // s is a reference to a String
    s.len()
}
// Here, s goes out of scope. But because it does not have ownership of what it refers to, nothing happens

fn look_ma_mutable_reference() {
    println!("begin look_ma_mutable_reference");
    let mut s = String::from("hello");
    let orig_s = s.clone();
    mutate_it(&mut s);
    println!("orig_s: \"{}\", s: \"{}\"", orig_s, s);
    println!("  end look_ma_mutable_reference");
}

fn mutate_it(s: &mut String) {
    s.push_str(", world");
}

fn slice_stuff() {
    println!("begin slice_stuff");
    let first = String::from("first word is first");
    let ix = first_word(&first);

    let slice = &first[..ix];
    println!("slice is \"{}\"", slice);

    let mut mut_first = String::from("mutate is the first word");
    let ix = first_word(&mut_first);
    let slice = &mut_first[..ix];
    // Hmm.  The string slice doesn't really help the "you captured the index but then the string changed" issue
    // Uncomment either mut_first.clear() and this code no longer compiles.
    // mut_first.clear();
    println!("slice is \"{}\"", slice);

    let slice = first_word_as_slice(&mut_first);
    println!("first_word_as_slice produces slice \"{}\"", slice);

    let slice = best_first_word(&mut_first);
    println!("best_first_word produces slice \"{}\"", slice);
    let slice = best_first_word("four score and seven");
    println!("best_first_word produces slice \"{}\"", slice);

    println!("  end slice_stuff");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_as_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn best_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
