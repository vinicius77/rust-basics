#![allow(unused)]

// Simple function
fn main() {
    println!("Functions");
    say_hello("Vinicius\n");

    let result = return_sum(12, 33);
    println!("Sum is {}\n", result);

    let implicit = implicit_return(22.40, 43.80);
    println!("Implicit return worked {}\n", implicit);
}

// function with arguments
fn say_hello(name: &str) {
    println!("Hello, {}", name);
}

// function that returns a value
fn return_sum(a: i32, b: i32) -> i32 {
    return a + b;
}

// Implicit return with a expression and without ";"
fn implicit_return(x: f64, y: f64) -> f64 {
    let result = x + y;
    result // no ; HERE
}
