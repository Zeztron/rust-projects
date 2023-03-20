use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();
    let c: i32 = 5;

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let parsed_guess = guess.trim().parse().expect("error");

    let d = add_two(c, parsed_guess);
    println!("d = {d}");
}

fn add_two(a: i32, b: i32) -> i32 {
    return a + b;
}
