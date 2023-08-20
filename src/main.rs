use std::io;

pub mod int_to_roman;

fn main() {

    println!("Please input an positive integer number:");

    let mut a = String::new();

    let _ = io::stdin()
        .read_line(&mut a)
        .expect("Failed to read line");

    let a: i64 = a.trim().parse().expect("Please type a number!"); // Shadowing a

    let result = int_to_roman::convert(a);

    let roman_number = match result {
        Ok(number) => number,
        Err(error) => panic!("Problem calculating Roman number: {:?}", error),
    };

    println!("The Roman notation for number {a} is {roman_number}");
}
