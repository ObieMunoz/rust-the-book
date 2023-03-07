fn main() {
    let config_max = Some(3u8);

    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    let config_max_2 = Some(5u8);
    if let Some(max) = config_max_2 {
        println!("The maximum is configured to be {}", max);
    }
}
