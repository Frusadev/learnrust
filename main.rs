use std::collections::HashMap;

fn main() {
    let mut ages: HashMap<String, u32> = HashMap::new();
    ages.insert(String::from("Help"), 32);
    match &ages.get("Help") {
        Some(value) => println!("Age {value}"),
        None => println!("No age found"),
    }

    let ages_vec = vec![1, 2, 3, 4, 5];
    for a in ages_vec.into_iter() {
        println!("{a}");
    }
    // Will fail
    if let Some(value) = ages_vec.get(0) {
        println!("{}", value)
    }
}
