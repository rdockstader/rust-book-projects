fn main() {
    let number = 3;

    // rust isn't a "truth" language, you must specify your condition
    // something like if number {} won't work.
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }


    // multiple branches
    if number % 4 == 0 {
        println!("number is divisable by 4");
    } else if number % 3 == 0 {
        println!("Number is divisable by 3");
    } else if number % 2 == 0 {
        println!("Number is divisable by 2");
    } else {
        println!("Number is not divisable by 2, 3, or 4!")
    }

    // inline if
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("the value of number is {number}");


    // loops
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result of the loop is {result}");

    // named loops and breaks
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
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
    println!("end count = {count}");


    // while loop
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");


    // for loops

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // or use a for loop, and get the same result:
    for element in a {
        println!("The value is {element}");
    }
    

    // this is liftoff, but with a for loop:
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!");
}

