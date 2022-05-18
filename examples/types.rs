#![allow(unused)]
fn main() {
    /** ====== Scalar types ====== */
    /* Integer
     ** i8 i32 i64 i132 => positive and negative number
     ** u8 u32 u64 u132 => only positive number
     ** isize usize => platform dependent (32bit x 64bit) */
    let x = 123;

    /* Float (DEFAULT is f64)
     ** f32 f64 f => positive and negative number */
    let float: f64 = 335.43;

    // CASTING
    let result_int: i32 = x * float as i32;
    let result_float: f64 = x as f64 * float;

    println!(
        "Types must match in numeric operations: integer {} and float {}\n",
        result_int, result_float
    );

    /* Boolean
     ** bool */
    let is_loading: bool = true;
    println!("Is loading? {}\n", is_loading);

    /* Char (one character)
     ** char */
    let emoji: char = '😃';
    println!("Are you happy? {}", emoji);

    /** ====== Compound types ===========
     * Tuple (Can have different types) */
    let coordinates = (22, 33, 9.3);
    println!("Coordinates: {:?}", coordinates);
    println!("Coordinate gravity: {:?}", coordinates.2);

    /* Arrays (Must have same type) FIXED LENGTH */
    let array = ["one", "two", "three"];
    println!("Array elements: {:?}", array);
    println!("Array last item {}", array[array.len() - 1]);

    // explicit typing
    let numbers_array: [i32; 2] = [12, 3];
    println!("Array numbers: {:?}", numbers_array);

    let six_twelves = [12; 6];
    println!("Initialize array with 12 six times: {:?}", six_twelves);
}
