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

## Functions

the `main` function is the entry point in Rust. `fn` keyword allows you to declare a new function. Rust code uses snake case for function and var names.

```Rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

You can call any function by entering its name followed by a pair of parenthesis. I'm going to create a project called `functions` in this repo to explore this further.

### Params

Params are defined within a function's `signature`. I'll add some params to another_function() - you must declare the type of each param in the function signature.

When defining multiple params, separate them with a `,`.

### Statements and Expressions

Function bodies are made up of a series of statements ending in an expression.
Rust is an expression based language.

- Statements are instructions that perform some action and do not return a value.
- Expressions evaluate to a resultant value.

Here is a difference between statements and expressions:

```Rust
fn main() {
    let y = 6; // a statement

    let y = (let x = 4); // if let x = 4 resulted in a return, this would work, it doesn't so error

    let x = random_number(); // random_number() is a function that ends in an expression, so will be able to assign a viable to it.
}
```

Calling a function is an expression, calling a macro is an expression, a new scope block created with `{}` is an expression.

```Rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };
    
    println!("The value of y is: {y}");
}
```

### Functions with return values

Functions can return values to the code that calls them. We must declare their type with an `->`. In Rust, the value of the return is synonymous with the value of the final line in a function. You can explicitly return with the `return` keyword as well.

if a line is an expression that you want to return, you do not end it with a `;`. If you end a function with a line that ends in a `;`, you won't return anything. 

## Comments

Single line comments / inline comments are done with two `//`, if you want a multi line one, you need to put it on each line.

## Control Flow

### if expressions

I'm going to create a new project called branches to explore this.

```Rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

Blocks of code associated with the conditions in `if` expressions are sometimes called `arms`, just like the `arms` in `match`.

#### Handling multiple conditions with else if

```Rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

It's probably best to use Rust's `match` if you have more than one `else if`.

#### Using an `if` in a `let` statement

```Rust
fn main() {
    let condition = true;

    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```

### Repetition with Loops

The `loop` keyword tells Rust to execute a block until told to stop.

you can put the `break` keyword into a loop to get Rust to break out of it when you want.

```Rust
fn main() {
    loop {
        println!("again!");
    }
}
```

Note - this is where I first noticed how quick Rust was.

#### Returning Values with Loops

To do this, you add the value you want returned after the `break`.

```Rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");
}
```

#### Disambiguating with Loop Labels

If you have nested loops, `break` and `continue` apply to the innermost loop at that point. You can optionall specify a `loop label` on a loop to then use with `break` or `continue`.

Loop labels begin with a singe quote `'`.

```Rust
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

#### Streamlining Conditional Loops with While

```Rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

#### Looping Through a Collection with `for`

```Rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

Most people would choose to do the previous while loop as a for loop over a range, like this:

```Rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!!");
}
```
