#![allow(unused)]

fn main() {
    println!("Hello, world! :)");

    // immutable string
    let imu_string: String = String::from("Vinicius");

    // mutable string
    let mut mutable_str: String = String::from("mutable");
    mutable_str = String::from("changed");

    // & pass the reference for the function not
    // the ownership of the variable
    println!("Imutable str {}", &imu_string);

    // if & is not passed, the imu_string cannot be
    // reused anywhere in the application e.g. below
    println!("Mutable string {} and {}", mutable_str, imu_string);
    println!(
        "Cannot be reused {}",
        mutable_str /*ERROR:, imu_string*/
    );

    say_hello(&mutable_str);
    say_hello(&mutable_str);
}

// Line 1 removes the unused warning
fn warning_example() {
    println!("!Warning!");
}

// reference not ownership (&)
fn say_hello(name: &String) {
    println!("Hello, {}", name);
}
