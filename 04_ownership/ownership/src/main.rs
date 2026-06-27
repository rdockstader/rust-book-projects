fn main() {
    // scope ownership
    {
        let _s = "hello"; //string literal -- only exists in scope {}

        // do stuff with s here
    }
    // s falls out of scope here

    // string type, can be mutable
    let mut s = String::from("Hello");

    s.push_str(", world!");

    println!("{s}");

    // scoped string
    {
        let _s = String::from("hello");

        // do stuff with s here. 
    }


    let x = 5;
    let y = x;
    // this binds both x and y to an independant value of 5, since they are
    // simple data types.
    println!("y={y} | x={x}");

    let str1 = String::from("hello");
    let str2 = str1;
    // this is different, because strings exist on the heap. 
    // str2 gets a copy of the pointer to the actual string on the heap
    // so they are pointing to the same data block

    println!("str2: {str2}, World!");

    // rust invalidates the first variable when this happens to prevent 
    // memory issues. So this line of code will fail:
    // println!("str1: {str1}, world!");

    // rust also deallocates as soon as something goes out of scope
    // as soon as hello turns into ahoy, hello gets deallocated
    {
        let mut s = String::from("hello");

        println!("{s}, world!");

        s = String::from("Ahoy");

        println!("{s}, world! I'm a new string!");
    }


    // cloning is how you get a deep copy of heap data
    {
        let s1 = String::from("Hello");
        let s2 = s1.clone();

        println!("s1 = {s1}, s2 = {s2}");
    }


    // functions take ownership of data on the heap but take a copy of 
    // data on the stack:

    {
        let s1 = String::from("hello");

        println!("{s1}, I'm a string!");
        takes_ownership(s1); // would need to clone if we want this used later
        // println!("{s1}, I'm a string!"); // wont work without clone

        let x = 5;

        println!("{x}, I'm an integer!");
        makes_copy(x);
        println!("{x}, I'm an integer!");
    }


    {
        let s1 = gives_ownership();

        println!("{s1} is mine now.");

        let s2 = String::from("sup dawg");

        let s3 = takes_and_gives_back(s2);

        println!("{s3} is printable, but s2 isn't");
    }


    // pass by reference example
    {

        let s1 = String::from("Hello, rust!");

        let len = calculate_length(&s1);

        println!("The length of '{s1}' is {len}.");
    }


}


fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}")
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string is now owned by this scope
    
    // return the string gives it back
    a_string


}


// you can pass by reference (not a pointer, a reference)
// a reference gurentees that the data will be there as long as you need it
// ...unlike a pointer...
fn calculate_length(s: &String) -> usize {
    s.len()
}
