# Fundamentals

```Rust
fun main() {

}
```

 This is always the first code that runs in every executable Rust program

 `println!()` is a macro, which is why it's got the `!`. 

 Always end lines with a semicolon, ;

in a cargo project, try cargo doc --open and it will open the documentation for all your dependencies in a webpage.

## Variables and mutability

```Rust
let x = 5; // immutable
let mut y = 5; // mutable variable
```

```Rust
let x: u32 = 15; // type is set to u32
```

```Rust
const ONE_HOUR_IN_SECONDS: u32 = 60 * 60; // convention is to name constants
```

Must use type annotation when declaring a constant
Constants are valid as long as are still in scope.

## Shadowing

```Rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2; // x is 12 here
    }
    // x is 6 here
}
```

another use for shadowing is to use the same name for a different type

```Rust
let spaces = "   ";
let spaces = spaces.len();
```

where as if we tried to do this to a mutating var, the compiler would cry.

## Data Types

### Scalar Types

represents a single value. like an int, float, bool, and char.

#### Numeric Operations

```Rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 -4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // results in -1

    // remainder
    let remainder = 43 % 5;
}
```

#### Bools

```Rust
fn main() {
    let t = true;

    let f: bool = false; // with type annotation
}
```

#### Char type

```Rust
fn main() {
    let c = 'z';
    let z: char = 'Z'; // with type annotation
    let heart_eyed_cat = '😻';
}
```

Chars are represented by single quotation marks.

### Compound Types

#### Tuples

```Rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

```Rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup; // this is called destructuring, as it breaks the tuple into 3 parts

    println!("the value of y is: {y}");
}
```

We can also access a part of a tuple using a `.` like so:

```Rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
```

#### Arrays

Arrays in Rust must be of a fixed length with all items being the same type.

```Rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

Arrays live on the stack - where as if you want to use something more flexible, use a vector, which lives on the heap.

For type annotation, you need to include the type of the elements, followed by the number of elements.

```Rust
fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
}
```

You can also initialize an array by having the first item in the type anotation being the element and the 2nd the number of times you want it repeated.

```Rust
fn main() {
    let a: [1; 6] // [1, 1, 1, 1, 1, 1];
}
```

##### Array element access

```Rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```
