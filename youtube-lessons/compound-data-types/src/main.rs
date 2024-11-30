// Compound Data Types
// arrays, tples, slices, and strings (slice string).
fn main() {
    //arrays - type specific either strings or ints
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number Array: {:?}", numbers);
    // ^ This works ^
    //  let mix: [i32; 5] = [1, 2, "apple", "horse", 5];
    // ^ This won't compile ^
    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits Array: {:?}", fruits);
    println!("Or if i just wnat one...");
    println!("My Favorite Fruit is a {}", fruits[1]);

    //tuples
    let human = ("Dallas", 29, false);
    // or i can define every data type...
    let human_tuple: (String, i32, bool) = ("Dallas".to_string(), 29, false);
    println!("Human Tuple A: {:?}", human);
    println!("Human Tuple B: {:?}", human_tuple);

    let intermix_tuple = ("Kratos", 1000, true, [1, 2, 3, 4, 5]);
    println!("The God of War: {:?}", intermix_tuple);

    //slices non interrupted

    let number_slice: &[i32] = &[1, 2, 3, 4, 5];
    println!("slice this {:?}", number_slice);

    let another_slice: &[&str] = &["Cats", "Dogs", "Elephants", "Bears"];
    println!("My favorite animals are a {:?}", another_slice);
}
