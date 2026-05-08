fn finbonacci(n: u64) -> u64 {
    if n < 2 {
        return n;
    } else {
        return finbonacci(n - 1) + finbonacci(n - 2);
    }
}

fn collatz_length(n: u64) -> u64 {

}

fn main() {
    let fib = finbonacci(12);
    println!("Fib: {fib}")
}
