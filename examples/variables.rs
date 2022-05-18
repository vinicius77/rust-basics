#![allow(unused)] // Crate attribute

fn main() {
    // immutable
    let x = 123;

    // mutable
    let mut number = 34;
    number = 35;

    // constant (explicit type)
    const DEFAULT_LEVEL: i32 = 40;

    // Shadowing (redefining variable with same name but completely different
    // value and / or type)
    let zeta = "123";
    let zeta: i32 = zeta.parse().unwrap();
    println!("Shadowing zeta {}\n", zeta);

    print!("Hello from Variables");
}
