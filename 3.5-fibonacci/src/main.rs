use std::io;

fn main() {
    println!("Which number in the fibonacci sequence do you want to find:");

    let mut fib_number = String::new();

    io::stdin()
        .read_line(&mut fib_number)
        .expect("Failed to read line");

    let fib_number: u32 = match fib_number.trim().parse() {
        Ok(num) => nth_fibonacci(num),
        Err(_) => 0,
    };

    println!("The answer is {fib_number}");
}

fn nth_fibonacci(x: u32) -> u32 {
    if x == 0 {
        0
    } else if x == 1 {
        1
    } else {
        nth_fibonacci(x - 1) + nth_fibonacci(x - 2)
    }
}

