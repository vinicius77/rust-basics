#![allow(unused)]
fn main() {
    print!("Control flow\n");

    /** IF / ELSE  */
    let num = 15;

    if (num < 5) {
        println!("Small Num");
    } else if num >= 5 && num <= 10 {
        println!("Average Num\n");
    } else {
        println!("Big number\n");
    }

    // implicit return
    let description = if (num < 5) {
        "Small Num"
    } else if num >= 5 && num <= 10 {
        "Average Num\n"
    } else {
        "Big number\n"
    };

    println!("Result {}", description);

    /** LOOP */
    let mut count = 0;

    loop {
        println!("Counting {} / 10", count);
        count += 1;
        if (count > 10) {
            break;
        }
    }

    /** WHILE */
    let fruits = ["apple", "banana", "caju", "morango"];
    // Shadowing / reset
    count = 0;

    while count <= fruits.len() - 1 {
        println!("Fruits I Like are {}\n", fruits[count]);
        count += 1;
    }

    /** FOR (in iter()) returns a reference & */
    for fruit in fruits.iter() {
        println!("Element {}\n", fruit);

        if (fruit == &"morango") {
            println!("{} is my favorite", fruit);
            break;
        }
    }

    /* FOR (in range) .. exclusive ..= inclusive */
    for number in (0..=12) {
        println!("{} in range of 12\n", number);
    }
}
