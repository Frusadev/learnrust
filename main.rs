fn finbonacci(n: u64) -> u64 {
    if n < 2 {
        return n;
    } else {
        return finbonacci(n - 1) + finbonacci(n - 2);
    }
}

fn collatz_length(n: u64) -> u64 {
    let mut current_length: u64 = 1;
    let mut current_val: u64 = n;
    if n == 0 {
        return 0;
    }
    while current_val != 1 {
        if current_val % 2 == 0 {
            current_val /= 2
        } else {
            current_val = 3 * current_val + 1;
        }
        current_length += 1;
    }
    current_length
}

fn main() {
    let fib = finbonacci(12);
    println!("Fib: {fib}");
    let cl = collatz_length(11);
    println!("Collatz Length: {cl}")
}
