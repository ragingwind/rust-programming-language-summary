// constants are always immutable
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

#[allow(unsafe_code)]
fn main() {
    // mutable variable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    {
        let x = x * 2;
        println!("The value of x in the inner scope is is: {}", x);
    }

    println!("The value of const is: {}", THREE_HOURS_IN_SECONDS);

    {
        // reuse the name of the variable
        let space = "    ";
        let space = space.len();
        println!("The value of space is: {}", space);
    }

    {
        let mut space = "    ";
        // compile-error: miss match type, re-assignment of immutable variable
        // space = space.len();
        println!("The value of space is: {}", space);
    }
}
