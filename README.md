# Common Programming Concepts
## Difference between `mut` and shadowing

|     | `mut`                                                                                                        | shadowing                                                                                                                                                                                              |
| --- | ------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 1.  | Allows to make a variable mutable, meaning you can change its value after it has been initialised, not type. | Allows to re-declare a variable with the same name in the same scope, effectively *shadowing* the previous variable, new variable can be of different type, mutable/immutable, regardless of original. |
| 2.  | Applies only to the specific variable within its current scope.                                              | Shadowed variable is hidden within the scope the new variable is declared. The previous variable is inaccessible until the shadowing variable goes out of scope.                                       |
| 3.  | You use `mut` when you need to modify the values of a variable multiple times throughout its lifetime.       | You use shadowing when you want to reuse a variable name but with a different type or value, or when you want to apply some transformation without mutating the original variable.                     |
## Data Types
Rust is a statically typed language, which means that it must know the types of all variables at compile time.

### Scalar data types
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

