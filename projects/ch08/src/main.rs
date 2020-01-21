fn main() {
    check_your_vector_victor();
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

    let v8_dash_5 = vec![1, 2, 3, 4, 5];
    println!("v8_dash_5: {:?}", v8_dash_5);

    let third_i = v8_dash_5[2];
    let third_ref_i: &i32 = &v8_dash_5[2];
    println!("third_i: {}, third_ref_i: {}", third_i, third_ref_i);
    /*
     * SO shows how to mutate that 3rd guy:  https://stackoverflow.com/questions/57449264/how-to-get-replace-a-value-in-rust-vec
     * Hmmm.  Somehow my third_ref_i example made my mutable thing immutable?
     */
    // std::mem::replace(&mut v8_dash_5[2], 33);
    // println!("third_i: {}, third_ref_i: {}, v8_dash_5: {:?}", third_i, third_ref_i, v8_dash_5);

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
}
