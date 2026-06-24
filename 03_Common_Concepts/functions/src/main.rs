fn main() {
    println!("Hello, world!");

    another_function();

    function_with_param(42);

    print_labeled_measurement(15, 'h');

    // this will result in an error because let 7 = 6 is a statement
    // and statements don't return any values
    // let x = (let y = 6):

    // this will work, because the x+1 is a expression and will return a value
    let y = {
        let x = 3;
        x + 1 // note that there is no semicolon ; here. 
    };
    // expressions do not have semicolons at the end of them.

    println!("The value of y is {y}");


    // return value function call

    let x = five();

    println!("The value of x is {x}");

    let x = plus_one(five());

    println!("The value of x is now {x}");
}

fn another_function() {
    println!("Hello from another function");
}

fn function_with_param(x: i32) {
    println!("The value of x is {x}")
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is {value}{unit_label}");
}

// function with a return value
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}