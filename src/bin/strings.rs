fn main() {
    let mut greeting = String::from("Hello World!");
    greeting.push_str(" Supriyo");

    println!("{}", greeting);
    let n = 4;

    if let Some(c) = greeting.chars().nth(n) {
        println!("{}", c);
    } else {
        println!("No character at position {}", n);
    }
}
