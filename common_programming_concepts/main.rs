fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let x = x + 1; // shadowed by x above

    {
        let x = x * 2; // shadowed by x above
        println!("the value of x in the inner scopes is: {x}");
    }

    println!("the value of x is: {x}");

    let spaces = "    ";
    let spaces = spaces.len(); //shadowing can help you even change datatypes
}