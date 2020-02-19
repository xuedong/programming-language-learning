// This is a recursive function.

fn recursive_factorial(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        n * recursive_factorial(n-1)
    }
}

fn main() {
    println!("Result: {}", recursive_factorial(10));
}
