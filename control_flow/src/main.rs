use std::io;

fn main() {
    println!("Hello, world!");

    loop {
        let choice = main_menu();

        match choice {
            0 => break,
            1 => input_celcius_output_fahrenheit(),
            2 => input_n_generate_fibo(),
            _ => continue, // has to be the last case
        }
    }
}

fn main_menu() -> i32 {
    println!("Menu");
    println!("1. Celcius to Fahrenheit conversion");
    println!("2. Fibonacci up to n-th");
    println!("0. Exit");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input.trim().parse().unwrap();
}

fn input_celcius_output_fahrenheit() {
    println!("Input Celcius: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let celcius: f64 = input.trim().parse().unwrap();
    let fahrenheit = celcius * 9.0 / 5.0 + 32.0;

    println!("{celcius} Celcius is {fahrenheit} Fahrenheit");
}

fn input_n_generate_fibo() {
    println!("Input n: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: i32 = input.trim().parse().unwrap();
    let mut a: i64 = 0;
    let mut b: i64 = 1;

    print!("Fibonacci up to {n}-th: ");
    for _ in 0..n {
        print!("{a} ");
        let c = a + b;
        a = b;
        b = c;
    }
    println!();
}