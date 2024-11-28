fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let q = 5;

    println!("original q is {q}");

    let q = q + 1;

    {
        let q = q * 2;
        println!("The value of my inner scoped q is: {q}")
    }

    println!("the value of the outer q is {q}");
}
