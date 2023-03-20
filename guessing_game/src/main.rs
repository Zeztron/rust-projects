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

    let arr = [1, 2, 3, 4, 5];
    let sum = sum_array(arr);
    println!("sum = {sum}");
}

fn add_two(a: i32, b: i32) -> i32 {
    return a + b;
}

fn sum_array(arr: [i32; 5]) -> i32 {
    let mut sum = 0;
    for i in arr.iter() {
        sum += i;
    }
    return sum;
}
