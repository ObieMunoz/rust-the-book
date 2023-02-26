fn main() {
    let s = String::from("hello world how are you?");

    let slice = &s[0..2];

    println!("{slice}");

    let slice = &s[..];

    println!("{slice}");
    
    let len = s.len(); 

    let slice = &s[0..len];

    println!("{slice}");

    println!("from function first_word: {}", first_word(&s));
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


