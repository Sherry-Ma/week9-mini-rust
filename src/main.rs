use std::io;

fn main() {
    println!("Enter a number: ");

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

    let num: u32 = num.trim().parse().expect("Please enter a number");

    let result = fib(num);

    println!("The {}th Fibonacci number is: {}", num, result);
}

fn fib(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }

    fib(n - 1) + fib(n - 2)
}
