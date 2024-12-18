# learning-rust

## Table of Contents

- [Common Programming Concepts](#common-programming-concepts)
  * [Difference between mut and shadowing](#difference-between-mut-and-shadowing)
  * [Data Types](#data-types)
    + [Scalar data types](#scalar-data-types)
    + [Compound Data Types](#compound-data-types)
  * [Functions](#functions)
    + [Statements and Expressions](#statements-and-expressions)
      - [Statements](#statements)
      - [Expressions](#expressions)
    + [Function with Return values](#function-with-return-values)
  * [Control Flow](#control-flow)
    + [if statement](#if-statement)
    + [Repetition with Loops](#repetition-with-loops)
- [Understanding Ownership](#understanding-ownership)
  * [What is Ownership](#what-is-ownership)
    + [Ownership rules](#ownership-rules)
    + [Variable scope](#variable-scope)
      - [The String type](#the-string-type)
      - [Memory and Allocation](#memory-and-allocation)
        * [Variables and Data Interacting with Move](#variables-and-data-interacting-with-move)
        * [Variables and Data Interacting with Clone](#variables-and-data-interacting-with-clone)
        * [Stack-Only Data: Copy](#stack-only-data--copy)
      - [Ownership and Functions](#ownership-and-functions)
      - [Return Values and Scope](#return-values-and-scope)
  * [Referencing and Borrowing](#referencing-and-borrowing)
    + [Mutable References](#mutable-references)
    + [Dangling References](#dangling-references)
  * [The Slice Type](#the-slice-type)
    + [Solution: String Slices](#solution--string-slices)
- [Using Structs to Structure Related Data](#using-structs-to-structure-related-data)
  * [Defining and Instantiating Structs](#defining-and-instantiating-structs)
    + [Tuple Structs](#tuple-structs)
    + [Unit-like structs without any fields](#unit-like-structs-without-any-fields)
  * [Method Syntax](#method-syntax)
    + [Where's the -> operator ?](#where-s-the----operator--)
- [Enums and Pattern Matching](#enums-and-pattern-matching)
  * [Defining an Enum](#defining-an-enum)
    + [Enum Values](#enum-values)
  * [The Option Enum and its Advantages Over Null Values](#the-option-enum-and-its-advantages-over-null-values)
  * [The match Control Flow Construct](#the-match-control-flow-construct)
    + [Patterns that bind to values](#patterns-that-bind-to-values)
    + [Matching with Option<T>](#matching-with-option-t-)
    + [Matches Are Exhaustive](#matches-are-exhaustive)
    + [Catch-all Patterns and the _ Placeholder](#catch-all-patterns-and-the---placeholder)
  * [Concise Control Flow with if let](#concise-control-flow-with-if-let)
    + [Option & Result](#option---result)
      - [Option](#option)
      - [Result](#result)
- [Managing Growing Projects with Packages, Crates and Modules](#managing-growing-projects-with-packages--crates-and-modules)
  * [Packages and Crates](#packages-and-crates)
  * [Defining Modules to Control Scope and Privacy](#defining-modules-to-control-scope-and-privacy)
  * [Paths for Referring to an item in the Module Tree](#paths-for-referring-to-an-item-in-the-module-tree)
  * [Bringing Paths Into Scope with the use Keyword](#bringing-paths-into-scope-with-the-use-keyword)
  * [Separating Modules into Different Files](#separating-modules-into-different-files)

<small><i><a href='http://ecotrust-canada.github.io/markdown-toc/'>Table of contents generated with markdown-toc</a></i></small>


## Common Programming Concepts
### Difference between mut and shadowing

|     | `mut`                                                                                                        | shadowing                                                                                                                                                                                              |
| --- | ------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 1.  | Allows to make a variable mutable, meaning you can change its value after it has been initialised, not type. | Allows to re-declare a variable with the same name in the same scope, effectively *shadowing* the previous variable, new variable can be of different type, mutable/immutable, regardless of original. |
| 2.  | Applies only to the specific variable within its current scope.                                              | Shadowed variable is hidden within the scope the new variable is declared. The previous variable is inaccessible until the shadowing variable goes out of scope.                                       |
| 3.  | You use `mut` when you need to modify the values of a variable multiple times throughout its lifetime.       | You use shadowing when you want to reuse a variable name but with a different type or value, or when you want to apply some transformation without mutating the original variable.                     |
### Data Types
Rust is a statically typed language, which means that it must know the types of all variables at compile time.

#### Scalar data types
Represents a single value.
1. Integers - numbers without fractional component.
   
| Length  | Signed | Unsigned |
| ------- | ------ | -------- |
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

   - Signed numbers are stored using 2's complement representation.
   - Each signed variant can store numbers from -(2<sup>n-1</sup>) to (2<sup>n-1</sup>) - 1 where *n* is the number of bits that variant uses.
   - Each unsigned variant can store numbers from 0 to 2<sup>n</sup> - 1 where *n* is the number of bits that variant uses.
   - Integer Literals in Rust
     

| Number Literals | Example |
| --------------- | ------- |
| Decimal | 98_222 |
| Hex	|0xff|
| Octal	|0o77|
| Binary	|0b1111_0000|
| Byte (u8 only)	|b'A'|

2. Floating-point numbers - numbers with fractional components.

```rust
let x = 2.0; // f64
let y: f32 = 3.0; // f32
```

3. Boolean type = 1 byte in size

```rust
let t = true;
let f: bool = false; // with explicit type annotation
```

4. Character type

```rust
let c = 'z';
let z: char = 'Z';  // with explicit type annotation
let heart_eyed_cat = '😻';
```

#### Compound Data Types

1. Arrays

```rust
let numbers: [i32; 5] = [1,2,3,4,5];
println!("Numbers Array: {:?}", numbers);

let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
println!("Fruits: {:?}", fruits);
println!("Fruits Array 1st Element: {}", fruits[2]);

/* 
Numbers Array: [1, 2, 3, 4, 5]
Fruits: ["Apple", "Banana", "Orange"]
Fruits Array 1st Element: Orange
*/
```

2. Tuples

```rust
let human:  (String, i32, bool) = ("Alice".to_string(), 30, false); // "Alice" is a string slice whereas we were expecting String
println!("Human Tuple: {:?}", human);
```

3. Slices

Refer to [The Slice Type](#the-slice-type).

4. Strings

Refer to [The String type](#the-string-type).


### Functions

- an function name/variables should be in snake case
- hoisting is ok in rust, you can create or call functions from anywhere in code.

```rust
fn another_function(x: i32) {     // x is a parameter, special variables that are part of a function's signature; concrete values are arguments.
   println!("the value of _x is {_x}");
}
```

#### Statements and Expressions

##### Statements 

- *Statements* are instructions that perform some action and do not return a value.
- Function definitions are also statements; the given example in its entirety is statement in itself.
- Statements do not return values, hence you can’t assign a let statement to another variable.

```rust
fn main() {
   let x = (let y = 6); // you can’t assign a let statement to another variable
}
```

##### Expressions

- *Expressions* evaluate to a resultant value.
- Calling a function or a macro is an expression; a new scope block created with curly brackets is an expression.
- Expressions do not incluee ending semicolons; if a semicolon is added to an expression it turns into a statement, and then it will not return a value.

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```

#### Function with Return values

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
/**
$ cargo run
Compiling functions v0.1.0 (file:///projects/functions)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
Running `target/debug/functions`
The value of x is: 5
*/
```

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

/**
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error[E0308]: mismatched types
 --> src/main.rs:7:24
  |
7 | fn plus_one(x: i32) -> i32 {
  |    --------            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
8 |     x + 1;
  |          - help: remove this semicolon to return this value

For more information about this error, try `rustc --explain E0308`.
error: could not compile `functions` (bin "functions") due to 1 previous error

 * */
```

### Control Flow

#### if statement

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

```

This is another example of using `if` in a `let` statement

```rust
fn main() {
    let condition = 4 > 5;
    let number = if condition { 4 } else { 5 };
    println!("The value of number is: {number}");
}
// The value of number is: 5
```

Although, this might result in error *`if` and `else` have incompatible types*.

```rust
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
}
```

#### Repetition with Loops

1. `loop` - Indefinitely runs a block of code, need explicit exit condition to `break`.
```rust
fn main() {
    loop {
        println!("again!");
    }
}
```
-  `break` can also be used to return values off `loop`.
    ```rust
    fn main() {
        let mut counter = 0;
    
        let result = loop {
            counter += 1;
    
            if counter == 10 {
                break counter * 2;
            }
        };
    
        println!("The result is {result}");
    }
    ```
- `loop` labels can be used to disambiguate between multiple loops.
    - If there are loops within loops, `break` and `continue` apply to the innermost loop at that point.
    - One can optionally specify a *loop label* on a loop that can be used with `break` and `continue` to specify that those keywords apply to the labelled loop instead of the innermost loop.
    - `loop` label must being with a single quote.
    ```rust
    fn main() {
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
        println!("End count = {count}");
    }
    ```
2. `while`
```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```
3. `for`
```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```

## Understanding Ownership
### What is Ownership
*Ownership* is a set of rules that govern how a Rust program manages memory. 
Rust uses an approach where memory is managed through a system of ownership with a set of rules that the compiler checks. 
If any of the rules are violated, the program won't compile. 

> **The stack and the heap** <br>
> The stack stores value in the order it gets them and removes the values in the opposite order (LIFO). All the data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead.
> The heap is less organized. While putting data on the heap: Request a certain amount of space -> memory allocator finds an appropriate empty spot in heap -> marks it as UNDER_USAGE -> returns a pointer for the spot -> perform book-keeping for the next allocation. (*Process is called allocation on heap, or just <u>allocation</u>, pushing values on stack is not considered allocation*). Although pointer to the empty spot of heap has a fixed size and can be stored on stack.
> Pushing to stack is always faster than pushing on heap, as location to push values is always at top of the stack. 
> Data access is also faster in stack than in heap, as location pointer has to be followed to access the value in the latter.
> When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.

#### Ownership rules
1. Each value in Rust has an *owner*.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the values will be dropped.

#### Variable scope

```rust
{                      // s is not valid here, it’s not yet declared
    let s = "hello";   // s is valid from this point forward

    // do stuff with s
}                      // this scope is now over, and s is no longer valid
```

##### The String type

- `String` type can be mutated `s2` from below whereas string literal `s` cannot be mutated.

```rust
let s = String::from("hello");
let mut s2 = String::from("hello");
s2.push_str(", world!"); // push_str() appends a literal to a String
println!("{s2}"); // This will print `hello, world!`
```

##### Memory and Allocation

In the above example, `String::from` requests the memory it needs. The memory is automatically returned once the variable that owns it goes out of scope.

```rust
{                                   // s is not valid here, it’s not yet declared
    let s = String::from("hello");  // s is valid from this point forward

    // do stuff with s
}                      // this scope is now over, and s is no longer valid
```

Here, we can return the memory of our `String` when `s` goes out of scope, at which point Rust calls a special function for us which is called `drop` automatically, which cleans up the heap memory for the variable.

> *This pattern of deallocating resources at the end of an item's lifetime is sometimes called Resource Acquisition Is Initialization (RAII), especially in C++.*

###### Variables and Data Interacting with Move

Let's take an example,

```rust
let x = 5; // bind 5 to x
let y = x; // make a copy of value in x and bind it to y
```

But what if we used string,

```rust
let s1 = String::from("hello");
let s2 = s1;
```

You see, `s1` is made up of 3 parts:
1. a pointer to the memory that holds the contents of the string;
2. length (how much memory in bytes the contents of the `String` are currently using);
3. capacity (total amount of memory in bytes that the `String` has received from the allocator)

These parts are stored on the stack, memory which holds the contents of the string is on the heap.

***When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack. We do not copy the data on the heap that the pointer refers to.***

**What if `s1` and `s2` both go out of scope ? Will they both try to free the same memory when `drop` is called ?**

This is called *double free error* which is a memory safety bug, leading to memory corruption.
Rust considers `s1` as no longer valid after line `let s2 = s1;`, meaning the following would result in an error:

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{s1}, world!");

/**
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:15
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |
5 |     println!("{s1}, world!");
  |               ^^^^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let s2 = s1.clone();
  |                ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership` (bin "ownership") due to 1 previous error
 * /
```

> *Rust does not do shallow copy here, as it also invalidated the first variable, hence it is called a **move**.*<br>
> *We would say `s1` is *moved* to `s2`.* <br>
> *Rust will never automatically create "deep" copies of the data, hence any copying can be assumed to be inexpensive in terms of runtime performance.*

###### Variables and Data Interacting with Clone

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {s1}, s2 = {s2}");

```

###### Stack-Only Data: Copy

```rust
let x = 5;
let y = x;

println!("x = {x}, y = {y}");
```
The above code is still valid.
The reason is that types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. That means there’s no reason we would want to prevent x from being valid after we create the variable y.

Rust has a special annotation called the `Copy` trait that we can place on types that are stored on the stack, as integers are. If a type implements the `Copy` trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.

Rust won’t let us annotate a type with `Copy` if the type, or any of its parts, has implemented the `Drop` trait. If the type needs something special to happen when the value goes out of scope and we add the `Copy` annotation to that type, we’ll get a compile-time error. 

**What types implement the `Copy` trait**

1. Any group of simple scalar values can implement `Copy`.
2. Nothing that requires allocation or is some form of resource can implement `Copy`.
    - All the integer types, such as u32.
    - The Boolean type, bool, with values true and false.
    - All the floating-point types, such as f64.
    - The character type, char.
    - Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

##### Ownership and Functions

The mechanics of passing a value to a function are similar to those when assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does.

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
```

##### Return Values and Scope

Returning values can also transfer ownership. 

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

### Referencing and Borrowing

The issue with the above code is we have to return the `String` to the calling function so we can still use the `String` after the call to `calculate_length`, because the `String` was moved into `calculate_length`. Instead, we can provide a reference to the `String` value. This action of creating a reference is called *borrowing*.

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // &s1 refers to s1 but does not own it, hence s1 will not be dropped.

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, it is not dropped.
```

> The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *. 

<span style="color:orange">We **<u>cannot modify</u>** something we are borrowing.</span> References are immutable just as variables.

```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
/**
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
 --> src/main.rs:8:5
  |
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
  |
help: consider changing this to be a mutable reference
  |
7 | fn change(some_string: &mut String) {
  |                         +++

For more information about this error, try `rustc --explain E0596`.
error: could not compile `ownership` (bin "ownership") due to 1 previous error

*/

```

#### Mutable References

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

Mutable references have one big restriction: <span style="color:orange">If you have a mutable reference to a value, you **<u>cannot have</u>** any other references to that value.</span>


```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);

/*
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:5:14
  |
4 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
5 |     let r2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
6 |
7 |     println!("{}, {}", r1, r2);
  |                        -- first borrow later used here

For more information about this error, try `rustc --explain E0499`.
error: could not compile `ownership` (bin "ownership") due to 1 previous error

*/
```

> The above restriction prevents something called a *data race*. A *data race* is similar to race condition, but happens when these 3  occur:
> 1. Two or more pointers access the same data at the same time.
> 2. At least one of the pointers is being used to write to the data.
> 3. There's no mechanism being used to synchronise access to the data.

Although, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:

```rust
let mut s = String::from("hello");

{
    let r1 = &mut s;
} // r1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;
```

Also, <span style="color:orange">If you have a mutable reference to a value, you **<u>cannot have</u>** any other mutable/immutable references to that value.</span>

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

println!("{}, {}, and {}", r1, r2, r3);

/*
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:14
  |
4 |     let r1 = &s; // no problem
  |              -- immutable borrow occurs here
5 |     let r2 = &s; // no problem
6 |     let r3 = &mut s; // BIG PROBLEM
  |              ^^^^^^ mutable borrow occurs here
7 |
8 |     println!("{}, {}, and {}", r1, r2, r3);
  |                                -- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ownership` (bin "ownership") due to 1 previous error

*/
```

<span style="color:orange">We also **<u>cannot</u>** have a mutable reference while we have an active scope immutable one to the same value.</span>

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{r1} and {r2}");
// variables r1 and r2 will not be used after this point

let r3 = &mut s; // no problem
println!("{r3}");
```

The scopes of the immutable references `r1` and `r2` end after the `println!` where they are last used, which is before the mutable reference `r3` is created; as the scopes don't overlap, this code is allowed.

#### Dangling References

A *dangling pointer* is one that references a location in memory that may be been given to someone else-by freeing some memory while preserving a pointer to that memory.
In Rust, this would never happen.

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!


/**
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0106]: missing lifetime specifier
 --> src/main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime, but this is uncommon unless you're returning a borrowed value from a `const` or a `static`
  |
5 | fn dangle() -> &'static String {
  |                 +++++++
help: instead, you are more likely to want to return an owned value
  |
5 - fn dangle() -> &String {
5 + fn dangle() -> String {
  |

error[E0515]: cannot return reference to local variable `s`
 --> src/main.rs:8:5
  |
8 |     &s
  |     ^^ returns a reference to data owned by the current function

Some errors have detailed explanations: E0106, E0515.
For more information about an error, try `rustc --explain E0106`.
error: could not compile `ownership` (bin "ownership") due to 2 previous errors

*/

// Solution here would to return the `String` directly.
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

TL;DR

- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.

### The Slice Type

Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
A slice is a kind of reference, so it does not have ownership.

```rust
fn first_word(s: &String) -> ?
```

We don't ownership, so this is fine. But what about return ? *We could return the index of the end of the word, indicated by space.*

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
```

There is a slight problem here. What if we emptied the argument `s` to empty string ?

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}
```

#### Solution: String Slices

A string slice is a reference to part of a `String`, and looks like this:

```rust
let s = String::from("hello world");

let hello = &s[0..5]; // string is truncated from start_index to (end_index-1).
let world = &s[6..11]; // start_index can be dropped [..11] and so can the end_index [3..] can also be written as [3..s.len()], and even both of them can be dropped [..]
```

The solution to the above would be:

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
```

Now this would not be even valid:

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {word}");
}

$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
  --> src/main.rs:18:5
   |
16 |     let word = first_word(&s);
   |                           -- immutable borrow occurs here
17 |
18 |     s.clear(); // error!
   |     ^^^^^^^^^ mutable borrow occurs here
19 |
20 |     println!("the first word is: {word}");
   |                                  ------ immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ownership` (bin "ownership") due to 1 previous error

```

> String literals are immutable and `&str` is an immutable reference.

```rust
fn first_word(s: &str) -> &str {

fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
```

## Using Structs to Structure Related Data

### Defining and Instantiating Structs

```rust
struct User {
    active: bool,
    username: String, // if we turn the type to &str, we would need lifetime specifier
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: Sring::from("someusername123"),
        email: String::from("someone@example.com"),
        sing_in_count: 1,
    }
    user1.email = String:
    
	let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };    
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
    
    // also we can write using field init shorthand
    // User {
    //     active: true,
    //     username,
    //     email,
    //     sign_in_count: 1,
    // }
}

```

#### Tuple Structs

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

#### Unit-like structs without any fields

You can also define structs that don't have fields.

> These can be useful when you need to implement a trait on some type but don't have any data that you want to store in the type itself.

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual; // no need for curly brackets or parentheses
}
```

**Adding Useful Functionality with Derived Traits**

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    }
    println!("rect is {}", rect1);
}

$ cargo run
Compiling structs v0.1.0 (/home/aditya-kumar/Work/src/github.com/aditya109/learning-rust/structs)
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
  --> src/main.rs:13:29
   |
13 |     println!("rect1 is {}", rect1);
   |                             ^^^^^ `Rectangle` cannot be formatted with the default formatter
   |
   = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

```

Hence the outer attribute,

```rust
#[derive(Debug)] // outer attribute
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    /**
     * output
     * rect1 is Rectangle { width: 30, height: 50 }
     */
    println!("rect1 is {rect1:?}");
    /**
     * 
     rect1 is Rectangle {
        width: 30,
        height: 50,
     }
     */
    println!("rect1 is {rect1:#?}");
}
```

Another way to print the struct object is:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/rectangles`
[src/main.rs:10:16] 30 * scale = 60
[src/main.rs:14:5] &rect1 = Rectangle {
    width: 60,
    height: 50,
}

```

### Method Syntax

Methods are similar to functions, but unlike functions methods are defined within the context of a struct (or an enum or a trait object), and their first parameter is always `self`, which represents the instance of the struct the method is being called on.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
```

#### Where's the -> operator ?

Rust does not have an equivalent to `->` operator, instead, it have *automatic referencing and dereferencing*.

Both of the following are same.

```rust
p1.distance(&p2);
(&p1).distance(&p2);
```

**More examples on methods**

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

All the function defined within an `impl` block are called *associated functions*. If the function defined within `impl` block does not have `self` as its first parameter.

We can also have multiple `impl` blocks, no reason why we need to do so, but we can, and it is valid.

```rust 
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

## Enums and Pattern Matching

*Enumerations* or enums allow you to define a type by enumerating its possible *variants*. 

### Defining an Enum

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

#### Enum Values

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

```rust'
fn route(ip_kind: IpAddrKind) {}

route(IpAddrKind::V4);
route(IpAddrKind::V6);
```

```rust
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
}

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
}
```

```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

The given enum `Message` has four variants with different types:

- `Quit` has no data associated with it at all.
- `Move` has name fields, like a struct does.
- `Write` includes a single `String`.
- `ChangeColor` includes three `i32` values.

```rust
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

// this is an equivalent structs
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

let sample = Message::Write(String::from("hello"));
let sample2 = Message::Quit;
let sample3 = Message::Move(12, 43);
let sample4 = Message::ChangeColor(234,567,234);
```

```rust
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

### The Option Enum and its Advantages Over Null Values

Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent. 

```rust
enum Option<T> {
    None, 
    Some(T),
}
```

The `Option<T>` enum is so useful; you don't need to bring it into scope explicitly. Its variants are also included in the prelude: `Some` and `None` can be directly used without the `Option::` prefix.

```rust
let some_number = Some(5);
let some_char = Some('e');

let absent_number: Option<i32> = None;
```
> Notes: `Option<T>` and `T` where `T` can be any type are different types, the compiler won't let us use an `Option<T>` value as if it were definitely a valid value.

### The match Control Flow Construct

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
fn main() {
    let coin = Coin::Penny;
    let value = value_in_cents(coin);
    println!("The value of the coin is: {} cents", value);
}

```

#### Patterns that bind to values

```rust
enum UsState {
    Albama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        },
    }
}

```

#### Matching with Option<T>

```rust
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```

#### Matches Are Exhaustive

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i+1),
    }
}
/*
$ cargo run
   Compiling enums v0.1.0 (file:///projects/enums)
error[E0004]: non-exhaustive patterns: `None` not covered
 --> src/main.rs:3:15
  |
3 |         match x {
  |               ^ pattern `None` not covered
  |
note: `Option<i32>` defined here
 --> /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/option.rs:571:1
 ::: /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/option.rs:575:5
  |
  = note: not covered
  = note: the matched value is of type `Option<i32>`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
  |
4 ~             Some(i) => Some(i + 1),
5 ~             None => todo!(),
  |

For more information about this error, try `rustc --explain E0004`.
error: could not compile `enums` (bin "enums") due to 1 previous error
*/
```

#### Catch-all Patterns and the _ Placeholder

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
```

Alternatively, you can use:

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
```

### Concise Control Flow with if let

```rust
fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
}
```

Alternatively, you could use

```rust
fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}
```

Exact analogy of `match`:

```rust
match my_variable {
    Some(x) => {
        println!("value is {}", x);
    },
    None => {
        println!("no value");
    }
}

match my_variable {
    _ => {
        println!("who cares");
    },
}

let x = match my_variable {
    Some(x) => x.squared() + 1,
    None => 42,
};
```

#### Option & Result

##### Option

```rust
let mut x: Option<i32> = None;
x = Some(5);
x.is_some();
x.is_none();

for i in x {
    println!("{}", i);
}
```

##### Result

```rust
#[must_use]

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```


```rust
use std::fs::File;

fn main() {
    File::open("foo");
}

/*
warning: unused `Result` that must be used
  --> src/main.rs:42:5
   |
42 |     File::open("foo");
   |     ^^^^^^^^^^^^^^^^^
   |
   = note: this `Result` may be an `Err` variant, which should be handled
   = note: `#[warn(unused_must_use)]` on by default
help: use `let _ = ...` to ignore the resulting value
   |
42 |     let _ = File::open("foo");
   |     +++++++
*/

// Correction:

use std::fs::File;

fn main() {
    let res = File::open("foo");
    let f = res.unwrap();
}

// output =>
// thread 'main' panicked at src/main.rs:43:17:
// called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }

// Correction:
use std::fs::File;

fn main() {
    let res = File::open("foo");
    let f = res.unwrap();
}

// output =>
// thread 'main' panicked at src/main.rs:43:17:
// error message: Os { code: 2, kind: NotFound, message: "No such file or directory" }
// stack backtrace:
//    0: rust_begin_unwind
//              at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/std/src/panicking.rs:662:5
//    1: core::panicking::panic_fmt
//              at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/core/src/panicking.rs:74:14
//    2: core::result::unwrap_failed
//              at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/core/src/result.rs:1677:5
//    3: core::result::Result<T,E>::expect
//              at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/core/src/result.rs:1059:23
//    4: enums::main
//              at ./src/main.rs:43:13
//    5: core::ops::function::FnOnce::call_once
//              at /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/core/src/ops/function.rs:250:5
// note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

// Correction:
use std::fs::File;

fn main() {
    let res = File::open("foo");
    if res.is_ok() {
        let f = res.unwrap();
    }
}

// match analogy
use std::fs::File;

fn main() {
    let res = File::open("foo");
    match res {
        Ok(f) => { /* do stuff */ },
        Err(e) => { /* do stuff */ },
    }
}

```

## Managing Growing Projects with Packages, Crates and Modules

- `Packages` - A Cargo feature that lets you build, test and share crates. Contains `Cargo.toml`.
- `Crates` - A tree of modules that produces a library or executable.
- `Modules and use` - Let you control the organization, scope and privacy of paths.
- `Paths` - A way of naming an item, such as a struct, function or module.

### Packages and Crates

A `crate` is the smallest amount of code that Rust compiler considers at a time. It can come in one of two forms: *binary crate* or *library crate*. <br/> 

*Binary crates* are programs you can compile to an executable that you can run, such as a command-line program or a server. Each must have a function called `main` that defines what happens when the executable runs. 

*Library crates* don't have a `main` function, and they don't compile to an executable.

> Usually by crate you mean *library crate*.
> The crate root is a source file that the Rust compiler starts from and makes upda the root module of your crate.

A *package* is a bundle of one or more crates that provides a set of functionality. A package contains a `Cargo.toml` file that describes how to build those crates. A package must contain at least one crate, whether that’s a library or binary crate.

Cargo follows a convention that `src/main.rs` is the crate root of a binary crate with the same name as the package. Likewise, Cargo knows that if the package directory contains `src/lib.rs`, the package contains a library crate with the same name as the package, and `src/lib.rs` is its crate root. Cargo passes the crate root files to rustc to build the library or binary.

If a package contains `src/main.rs` and `src/lib.rs`, it has two crates: a binary and a library, both with the same name as the package. A package can have multiple binary crates by placing files in the `src/bin` directory: each file will be a separate binary crate.

### Defining Modules to Control Scope and Privacy

`use` keyword brings a path into scope; `pub` keyword is used to make items public

#### Modules internal working

- **Start from the crate root**: When compiling a crate, the compiler first looks in the crate root file (usually *src/lib.rs* for a library crate or *src/main.rs* for a binary crate) for code to compile.

- **Declaring modules**: In the crate root file, you can declare new modules; say you declare a “garden” module with `mod garden;`

  . The compiler will look for the module’s code in these places:

  - Inline, within curly brackets that replace the semicolon following `mod garden`
  - In the file *`src/garden.rs`*
  - In the file *`src/garden/mod.rs`*

- **Declaring submodules**: In any file other than the crate root, you can declare submodules. For example, you might declare `mod vegetables;` in `src/garden.rs`. The compiler will look for the submodule’s code within the directory named for the parent module in these places:

  - Inline, directly following `mod vegetables`, within curly brackets instead of the semicolon
  - In the file *`src/garden/vegetables.rs`*
  - In the file *`src/garden/vegetables/mod.rs`*

- **Paths to code in modules**: Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code. For example, an `Asparagus` type in the garden vegetables module would be found at `crate::garden::vegetables::Asparagus`.

- **Private vs. public**: Code within a module is private from its parent modules by default. To make a module public, declare it with `pub mod` instead of `mod`. To make items within a public module public as well, use `pub` before their declarations.

- **The `use` keyword**: Within a scope, the `use` keyword creates shortcuts to items to reduce repetition of long paths. In any scope that can refer to `crate::garden::vegetables::Asparagus`, you can create a shortcut with `use crate::garden::vegetables::Asparagus;` and from then on you only need to write `Asparagus` to make use of that type in the scope.

#### Grouping Related Code in Modules

To create a new library `restaurant` by running `cargo new restaurant --lib`.

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

**By using modules, we can group related definitions together and name why they’re related.**

Module tree for the structure above:

```bash
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

> A *nested* module and *submodule* are essentially the same thing; the distinction is mostly about perspective.

### Paths for Referring to an item in the Module Tree

A path can take two forms:

- An *absolute path* is the full path starting from a crate root:
  - For code from an external crate, the absolute path begins with the crate name. ` crate::front_of_house::hosting::add_to_waitlist(); `
  - For code from the current crate, it starts with the literal `crate`. 
- A *relative path* starts from the current modules and uses `self`, `super` or an identifier in the current module. `front_of_house::hosting::add_to_waitlist();`

```rust
Directory Structure:

└── ./
    └── modules_2
        └── src
            ├── front_of_house
            │   ├── hosting.rs
            │   ├── mod.rs
            │   └── serving.rs
            └── main.rs



---
File: /modules_2/src/front_of_house/hosting.rs
---

pub fn add_to_waitlist() {
    println!("Adding to waitlist.");
}
fn seat_at_table() {
    println!("Seating at table.");
}


---
File: /modules_2/src/front_of_house/mod.rs
---

pub mod hosting;
mod serving;

fn take_order() {
    println!("taking order.");
}



---
File: /modules_2/src/front_of_house/serving.rs
---

fn take_order() {
    println!("taking order");
}

fn serve_order() {
    println!("serving order");
}

fn take_payment() {
    println!("taking payment");
}


---
File: /modules_2/src/main.rs
---

mod front_of_house;

fn main() {
    // Access the public `add_to_waitlist` function
    front_of_house::hosting::add_to_waitlist();

    // The following line would cause an error because `seat_at_table` is private
    // front_of_house::hosting::seat_at_table();

}
```

#### Now nitpicks for the above ?

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist(); // module `hosting` is private

    // Relative path
    front_of_house::hosting::add_to_waitlist(); // module `hosting` is private
}
```

> Items in a parent module can’t use the private items inside child modules, but items in child modules can use the items in their ancestor modules. 
>
> This is because child modules wrap and hide their implementation details, but the child modules can see the context in which they’re defined. 
>
> To continue with our metaphor, think of the privacy rules as being like the back office of a restaurant: what goes on in there is private to restaurant customers, but office managers can see and do everything in the restaurant they operate.

```rust
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {} 
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist(); // function `add_to_waitlist` is private

    // Relative path
    front_of_house::hosting::add_to_waitlist(); // function `add_to_waitlist` is private
}
```

> Adding the `pub` keyword in front of `mod hosting` makes the module public. 
>
> With this change, if we can access `front_of_house`, we can access `hosting`. 
>
> But the *contents* of `hosting` are still private; making the module public doesn’t make its contents public. 
>
> The `pub` keyword on a module only lets code in its ancestor modules refer to it, not access its inner code. 
>
> Because modules are containers, there’s not much we can do by only making the module public; we need to go further and choose to make one or more of the items within the module public as well.

```rust
// correction
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

#### Starting relative paths with super

We can construct relative paths that begin in the parent module, rather than the current module or the crate root, by using `super` at the start of the path. 

```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

#### Making Structs and Enums Public

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
```

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

```



### Bringing Paths Into Scope with the use Keyword

### Separating Modules into Different Files

