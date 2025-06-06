fn main() {
    // Normal for loop
    for i in 0..5 {
        println!("i: {}", i);
    }

    // Setting step
    for i in (0..100).step_by(10) {
        println!("{}", i);
    }

    // While loop
    let mut x = 1;
    while x <= 5 {
        println!("{}", x);
        x += 1;
    }
}
