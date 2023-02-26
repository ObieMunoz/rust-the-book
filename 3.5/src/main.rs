use std::io;

fn main() {
    println!("Enter a number:");

    loop {
        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line.");

        let number: i32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        condition(number);
    }
}

fn condition(number: i32) {
    if number == 5 {
        println!("number is 5!");
    } else if number < 5 {
        println!("number is less than 5");
    } else {
        println!("number is greater than 5");
    }
}
