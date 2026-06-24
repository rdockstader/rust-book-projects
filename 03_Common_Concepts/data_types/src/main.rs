fn main() {
    let x = 2.0; // f64 (64 bit float)

    let y: f32 = 3.0; // f32 (32 bit float)

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division

    let quotient = 56.7 / 32.2;

    let truncated = -5 / 3; // results in -1

    // remainder
    let remainder = 43 % 5;

    // bool (implicit)
    let t = true;

    // bool (explicit)

    let f: bool = false;

    //character type

    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻'; // char type uses unicode scalar values instead of standard ASCII. Allows for more character types then just alphanumeric. 

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    let five_hundred = tup.0;


    // array
    let a = [1, 2, 3, 4, 5];

    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sept", "Oct", "Nov", "Dec"];

    let b: [i32, 5] = [1, 2, 3, 4, 5];

    // initialize an array of size, where all elements are the same value:
    let fives_array = [5; 5]; // [5, 5, 5, 5, 5]

    let first_element = months[0];
    let second_element = months[1];

    

}
