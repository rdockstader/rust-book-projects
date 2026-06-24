
// just because let keyword makes things immutable, doesn't mean it makes them constant.
// this is a constant:
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // this is ran at compile time, so it's not violating the const declaration


fn main() {
    let mut x = 5; // with the MUT keyword, the rest of this phrase will fail
    println!("The value of x is {x}");
    x = 6;
    println!("THe value of x is {x}");



    // shadowing -- replacing an immutable variable with a new value is called shadowing, and requires the let keyword

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is {y}");
    }

    println!("The value of y in the outser scope is {y}");
}
