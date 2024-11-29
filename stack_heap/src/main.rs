fn main() {
    let s = "hello"; // first in, last out
                     //hardcoded into the stack

    let mut t = String::from("this is a string"); //no using memory and we need to be
                                                  //careful.
                                                  // sorta like a mutable string
    t.push_str(", world!");
    // THIS WAS APPENDED LIKE AN ARRAY IN JS

    println!("{s} {t}");
} // no that t is out of scope the memory is released.
