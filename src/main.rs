use crate::math;

fn main() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 4, 5, 6];
    let result: Vec<i32> = numbers
        .iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * x)
        .collect();
    for n in &numbers {
        println!("{n}")
    }
    println!("------------------------------------------");
    for n in &result {
        println!("{n}")
    }
}
