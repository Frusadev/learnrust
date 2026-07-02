fn main() {
    let mut numbers = vec![1, 2, 3, 4];
    numbers.push(12);
    numbers.push(32);

    let first = numbers.get(0);
    match &first {
        Some(value) => println!("This is the first value: {}", value),
        None => println!("No value found"),
    }

    for number in &numbers {
        println!("{number}");
    }

    for number_to_square in numbers.iter_mut() {
        *number_to_square *= *number_to_square;
    }

    for number in numbers {
        println!("{number}")
    }
}
