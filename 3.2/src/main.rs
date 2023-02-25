fn main() {
    let x = 2.0;
    let y: f32 = 3.0;

    println!("x = {}, y = {}", x, y);

    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;

    let floored = 2 / 3;

    let remainder = 43 % 5;

    println!("sum = {}", sum);
    println!("difference = {}", difference);
    println!("product = {}", product);
    println!("quotient = {}", quotient);
    println!("floored = {}", floored);
    println!("remainder = {}", remainder);

    let t = true;

    let f: bool = false;

    println!("t = {}, f = {}", t, f);

    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';

    println!("c = {}, z = {}, heart_eyed_cat = {}", c, z, heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _z) = tup; // _ is used to ignore the value otherwise we will get a compiler
                           // warning

    println!("The value of y is: {}", y);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!(
        "five_hundred = {}, six_point_four = {}, one = {}",
        five_hundred, six_point_four, one
    );

    let a = [1, 2, 3, 4, 5];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let first = months[0];
    let second = months[1];

    println!("The first month is {} and the second month is {}", first, second);

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let arr = [3; 5];
}
