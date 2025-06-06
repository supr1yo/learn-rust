fn main() {
    /*
        let mut greeting = String::from("Hello World!");
        greeting.push_str(" Supriyo");

        println!("{}", greeting);
        let n = 4;

        if let Some(c) = greeting.chars().nth(n) {
            println!("{}", c);
        } else {
            println!("No character at position {}", n);
        }
    */
    let sentence = String::from("My name is Supriyo");
    let first_word = get_first_word(sentence);
    print!("First word is: {}", first_word);
}

fn get_first_word(sentence: String) -> String {
    let mut ans = String::new();

    for char in sentence.chars() {
        if char == ' ' {
            break;
        }
        ans.push(char);
    }

    return ans;
}
