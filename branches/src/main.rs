fn main() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4,3 or 2");
    }
    conditional_let();
}

fn conditional_let() {
    let condition = 4 > 5;
    let number = if condition { 4 } else { 5 };
    println!("The value of number is: {number}");
}
