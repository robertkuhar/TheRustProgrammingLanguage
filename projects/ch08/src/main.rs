fn main() {
    check_your_vector_victor();
    string_theory();
    na_na_na_na_na_na_na_na_hash_map();
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn check_your_vector_victor() {
    let mut v0: Vec<i32> = Vec::new();
    v0.push(1);
    v0.push(2);
    v0.push(3);
    println!("v0: {:?}", v0);
    v0.pop();
    println!("v0: {:?}", v0);
    v0.remove(0);
    println!("v0: {:?}", v0);

    /*
     * SO shows how to mutate that 3rd guy:  https://stackoverflow.com/questions/57449264/how-to-get-replace-a-value-in-rust-vec
     * but it seems there is an easier way
     */
    let value_at_index_0 = &mut v0[0];
    *value_at_index_0 = 4;
    println!("v0: {:?}", v0);

    let v8_dash_5 = vec![1, 2, 3, 4, 5];
    println!("v8_dash_5: {:?}", v8_dash_5);

    let third_i = v8_dash_5[2];
    let third_ref_i: &i32 = &v8_dash_5[2];

    println!("third_i: {}, third_ref_i: {}", third_i, third_ref_i);
    match v8_dash_5.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    for i in &v8_dash_5 {
        println!("i: {}", i);
    }

    for i in v8_dash_5 {
        println!("i: {}", i);
    }

    let mut v = vec![100, 32, 57];
    println!("The reference data type just got more confusing. before, v: {:?}", v);
    for i in &mut v {
        *i += 50;
    }
    println!("The reference data type just got more confusing.  after, v: {:?}", v);

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    // I had to #[derive(Debug)] on SpreadsheetCell to get this to work
    println!("{:?}", row);
    for cell in row {
        match cell {
            SpreadsheetCell::Int(x_int) => println!("x_int is {}", x_int),
            SpreadsheetCell::Text(x_text) => println!("x_text is {}", x_text),
            SpreadsheetCell::Float(x_float) => println!("x_float is {}", x_float)
        }
    }
}

fn string_theory() {
    let mut hello = String::from("hello");
    hello.push_str(", Olá");
    hello.push_str(", Γειά σου");
    println!("hello: {}", hello);

    let hello_scottish_galic = String::from("Halò");
    hello.push_str(", ");
    hello.push_str(&hello_scottish_galic);
    println!("hello: {}, hello_scottish_galic: {}", hello, hello_scottish_galic);

    let hello_icelandic = "Halló";
    hello.push_str(", ");
    hello.push_str(hello_icelandic);
    println!("hello: {}", hello);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s: {}, s1: {}, s2: {}, s3: {}", s, s1, s2, s3);
}

use std::collections::HashMap;

fn na_na_na_na_na_na_na_na_hash_map() {
    let mut scores_1 = HashMap::new();
    scores_1.insert(String::from("Blue"), 10);
    scores_1.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores_2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let team_name = String::from("Blue");
    let blue_score_1 = scores_1.get(&team_name);
    let blue_score_2 = scores_2.get(&team_name);
    let blue_score_3 = scores_2.get(&String::from("steelers"));

    println!("blue_score_1: {:?}, blue_score_2: {:?}", blue_score_1, blue_score_2);
    // How do I println! None in rust
    match blue_score_3 {
        Some(p) => println!("blue_score_3: {}", p),
        None => println!("blue_score_3 is None"),
    }

    println!("Before");
    for (key, value) in &scores_1 {
        println!("\tscores_1.get({}): {}", key, value);
    }
    scores_1.insert(String::from("Blue"), 25);
    println!("Blue to 25");
    for (key, value) in &scores_1 {
        println!("\tscores_1.get({}): {}", key, value);
    }

    println!("Blue gets 50 if it isn't already in map.\n5 Points to Gryffindor.");
    scores_1.entry(String::from("Blue")).or_insert(50);
    let gryffindor = scores_1
        .entry(String::from("Gryffindor"))
        .or_insert(5);

    println!("5 Points to Gryffindor.");
    *gryffindor += 5;

    for (key, value) in &scores_1 {
        println!("\tscores_1.get({}): {}", key, value);
    }
    let text = "hello world wonderful world";
    println!("word_distribution({}) -> {:?}", text, find_word_distribution(text));
}

fn find_word_distribution(text: &str) -> HashMap<&str, i32> {
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    return map;
}

/* I can't write this function.  The fuck?
fn dump_map(format_str: String, hash_map: &HashMap<String,i32>) {
    for (key, value) in &hash_map {
        println!(format_str, key, value);
    }
}
*/
