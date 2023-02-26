fn main() {
    println!("Hello, world!");

    another_function(5, 'h');
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let z = return_value();

    println!("The value of z is: {z}");

    let num = plus_one(1);
    println!("The num is {num}");
}

fn another_function(value: i32, unit_label: char) {
    println!("Another function.");
    println!("The measurement is {value}{unit_label}.");
}

fn return_value() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
