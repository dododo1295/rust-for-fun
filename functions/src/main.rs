fn main() {
    let x = five();
    let plus = five_plus(x);

    println!("The value of x is: {x}");

    print_labeled_measurement(5, 'h');

    println!("check this out, I have {x}, and then I can go like this and make {plus}");
}

// My extra functions location doesn't matter, can be before or after, BUT in order to be used it
// must be put into the <main> function.
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
fn five() -> i32 {
    5
}
// in order to return, must omit the ; from the code line
fn five_plus(x: i32) -> i32 {
    x + 1
}
