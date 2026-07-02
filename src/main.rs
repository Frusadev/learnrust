mod math;
mod process;
use crate::math::add;

fn main() {
    println!("Hello, world!");
    let added = add(&1, &2);
    println!("{added}")
}
