// Primitive data types
// int, float, bool, char
// Rust has signed (+ and -) and unsigned (only+) integer types of different sizes.
// i8, i16, i32, i64, i128, isize: signed integers. -(2^n-1) to (2^n-1)
// u8, u16, u32, u64, u128, usize: unsigned integers. 0 to (2^n)-1
fn main() {
    let x: i32 = -42;
    let y: u64 = 100; //must be positive.

    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

    // diff bit sizes i##, where # equals bit size;
    // ranges:
    // i32 <= 2147483647
    // i64 <= 9223372036854775807
    let e: i32 = 2147483647;
    let i: i64 = 9223372036854775807;
    println!("Maximum value of i32: {}", e);
    println!("Maximum value of i64: {}", i);

    // Floats [Floating Point Types]
    // f32, f64
    let pi: f64 = 3.14;
    println!("Value of pi: {}", pi);

    //Booleans
    let is_snowing: bool = true;
    println!("Is it snowing?: {}", is_snowing);

    //Character Type - char
    let letter: char = 'a';
    println!("First letter of the alphabet is '{}'.", letter);
}
