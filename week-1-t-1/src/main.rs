use std::io;

fn main() {
    println!("Enter how many Fibonacci terms to generate (max 45):");

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        println!("Failed to read input.");
        return;
    }

    let num: u32 = match input.trim().parse() {
        Ok(n) if n <= 45 => n,
        Ok(_) => {
            println!("Please enter a number up to 45.");
            return;
        }
        Err(_) => {
            println!("Invalid input. Please enter a non-negative number.");
            return;
        }
    };

    println!("\nFibonacci sequence ({} terms):\n", num);
    println!("{:<6} | {:>20}", "Index", "Value");
    println!("-------------------------------");

    for i in 0..num {
        println!("{:<6} | {:>20}", i, fibonacci(i));
    }
}

/// Recursive function to calculate Fibonacci number at position `n`
fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
