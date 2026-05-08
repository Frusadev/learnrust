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

fn transpose(m: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut final_matrix: [[i32; 3]; 3] = [
        [0, 0, 0], // lol
        [0, 0, 0],
        [0, 0, 0],
    ];
    for i in 0..3 {
        for x in 0..3 {
            final_matrix[i][x] = m[x][i];
        }
    }
    final_matrix
}

fn magnitude(vec3: &[f64; 3]) -> f64 {
    let mut s: f64 = 0.0;
    for p in vec3 {
        s += p * p
    }
    s.sqrt()
}

fn normalize(vector: &mut [f64; 3]) {
    let mg = magnitude(vector);
    for item in vector {
        *item /= mg;
    }
}

fn main() {
    let fib = finbonacci(12);
    println!("Fib: {fib}");
    let cl = collatz_length(11);
    println!("Collatz Length: {cl}");
    let matrix_to_transpose = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];
    println!("Original:");
    for row in matrix_to_transpose {
        println!("{row:?}")
    }
    let transposed = transpose(matrix_to_transpose);
    println!("Transposed: ");
    for row in transposed {
        println!("{row:?}")
    }

    let arr = [1, 2, 3, 4, 5];
    let s: &[i32] = &arr[1..3];

    println!("\nSlice test");
    println!("{s:?}");

    println!(
        "Magnitude of a unit vector: {}",
        magnitude(&[0.0, 1.0, 0.0])
    );

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}
