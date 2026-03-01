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

