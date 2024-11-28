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

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
    // labelling loops to disambiguate between multiple loops.
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

    //infinite loops

    // loop {
    //    println!("AGAAAAAINNN");
    // }

    // while loops

    let mut number_thrice = 3;

    while number_thrice != 0 {
        println!("{number_thrice}");

        number_thrice -= 1;
    }
    println!("LIFTOFF!!");

    // loop with for (Slow Compile time)
    let ah = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", ah[index]);

        index += 1;
    }
    // OR to be more concise I can.... (way safer from array resizing and speedier in run time)

    let ahh = [10, 20, 30, 40, 50];

    for element in ahh {
        println!("the value is: {element}");
    }
    // how about a litle reving booooiiiisss

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF....er....again!");
}
