fn main() {
    let s = "This is"; // first in, last out
                       //hardcoded into the stack

    let mut t = String::from("a string"); //no using memory and we need to be
                                          //careful.
                                          // sorta like a mutable string
    t.push_str(", and this is pushed on top!");
    // THIS WAS APPENDED LIKE AN ARRAY IN JS

    println!("{s} {t}");

    let pointee = String::from("this is in memory");
    let pointer = pointee; //  NOTE: this isn't copying pointee over, instead it's using the
                           // same pointer to the string on the stack, not the heap.
                           //  NOTE: double free memories can happen because both access the same data so when it is freed
                           //  the both try to <drop> the same data. rust has safety to clear pointee after it's data is
                           //  reassigned to to pointer. cool beans.
    println!("{pointer}");

    let here_one = String::from("Hello");
    let here_two = here_one.clone();
    println!("here_one : {here_one}, and now copied from the heap, here_two: {here_two}");
    //  NOTE: .clone() "deeply" copies the entire data from the heap.
    //  NOTE: when .clone() is used code is taken from the heap and is used in a more memory heavy
    //  way. visual indicator that something different is happening from using the pointer.

    let x = 69;
    let y = x;
    println!("{x}, {y}");
    // NOTE: with integers it copies direct from the stack and both can be called. integers have a
    // known size at compile time so it doesn't make a difference.

    let going_out = String::from("Hello");

    its_out(going_out);

    let num = 42;

    num_out(num);
} //  NOTE: that t is out of scope the memory is released. this is a function of rust so there is no
  // GC.

fn its_out(some_string: String) {
    println!("{some_string}, I've now changed scopes");
    // NOTE: now that going_out is called out of main()'s scope I can't bring it back out, it's
    // dropped. IF
}

fn num_out(some_integer: i32) {
    println!("{some_integer} is the meaning of life.")
    // NOTE: while I've pulled num into this function I can still call on num in main() without a
    // run or compile-time error.
}
