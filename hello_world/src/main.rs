fn main() {

    // mutability
    let mut _x = 5;
    println!("the value of _x is {_x}");
    _x = 6;
    println!("the value of _x now is {_x}");

    // constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // shadowing
    let _y = 5;

    let _y = _y + 1;

    {
        let _y = _y * 3;
        println!("the value of _y is {_y}");
    }
    println!("the value of _y is {_y}");

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    let t = true;
    let f: bool = false; // with explicit type annotation

    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Numbers Array: {:?}", numbers);
    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits: {:?}", fruits);
    println!("Fruits Array 1st Element: {}", fruits[2]);

    let human:  (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human Tuple: {:?}", human);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
