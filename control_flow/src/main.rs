fn main() {
    // if conditions

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    };

    if number >= 3 {
        println!("hey this was higher than {number}");
    } else if number == 3 {
        println!("hey its 3, good job");
    } else {
        println!("wowza, this is lower than 3");
    };
    // can also add if's into a let.

    let condition = true;
    let number_again = if condition { 5 } else { 6 };
    println!("The value of number is: {number_again}");

    // loops

    let mut counter = 0;

    let result = 'counting_up: loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // while loops
}
