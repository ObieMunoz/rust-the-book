#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "Method 1: The area of the rectangle is {} square pixels.",
        area_1(width1, height1)
    );

    let rect1 = (30, 50);

    println!(
        "Method 2: The area of the rectangle is {} square pixels.",
        area_2(rect1)
    );

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "Method 3: The area of the rectangle is {} square pixels.",
        area_3(&rect2)
    );

    println!("rect2 is {:#?}", rect2);

    let scale = 2;

    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect3);
}

fn area_1(width: u32, height: u32) -> u32 {
    width * height
}

fn area_2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
