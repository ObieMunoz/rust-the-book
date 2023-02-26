use std::io;

fn main() {
    loop {
        println!("Let's convert temperatures from Fahrenheit to Celsius!");
        println!("Please enter a temperature in Fahrenheit:");

        let mut fahrenheit = String::new();
        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Failed to read line");

        let fahrenheit: i32 = match fahrenheit.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        let celsius = (fahrenheit - 32) * 5/9;

        println!("The temperature in celsius is {celsius}!");
    }
}
