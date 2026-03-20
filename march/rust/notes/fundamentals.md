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

## Structs

A way to group data

### Defining and Instantiating Structs

To define a struct, you enter the keyword `struct` and give it a name. It should describe the significance of the data being grouped together. Then, inside curly braces, we define the names and types of data, called fields.

```Rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

To use a struct, we define an instance of one and specify concrete values for each of the fields using curly braces containing `key: value` pairs.

```Rust
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123)"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    }l
}
```

To get a specific value, we use dot notation. For example: 

```Rust
user1.email = String::from("anotheremail@example.com");
```

Note: the entire instance must be mutable, you can't only mark certain fields as mutable.

```Rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count = 1,
    }
}
```

This is a cumbersome way to define default values.

#### Using the Field Init Shorthand

Because the params and the struct field names have the exact same names, we can use the field init shorthand.

```Rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

#### Creating Instances With Struct Update Syntax

It's often useful to create a new instance of a struct that includes most of the values from another instance of that struct.

To do this the regular way, this is what it would look like:

```Rust
fn main() {
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}
```

Now using struct update syntax, it is much easier - the syntax `..` specifies that the remaining fields should be given the same values as the given instance.

```Rust
fn main() {
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

It is worth noting that struct update syntax moves the data, so we can't use `user1` after this. If we gave `user2` a new string for its username and only used the update for `active` and `sign_in_count`, then we could still use `user1`.

#### Creating Different Types with Tuple Structs

Tuple structs don't have names associated with their fields, just the type.

To define a tuple struct, start with the `struct` keyword, followed by the name, and then the tuple with its types as the values.

```Rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

Note: black and origin are different because they are of different struct types.

#### Defining Unit-Like Structs

A struct without any fields is called a unit like struct. They behave similarly to (). They are useful when you want to implement a trait on a particular type, but don't have any data you want to associate with it.

```Rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```


### Methods

```Rust
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

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    )
}
```

We use a `impl` / implementation block for `Rectangle` to define a method. Everything in the `impl` block is associated with the `Rectangle` struct.

Methods must have their first param name either `self` of type `Self`, so Rust lets you shorten this to only using `self`. We still put the `&` to indicate that we're borrowing.

Having a method that takes ownership of `self` in the params is rare.

We can choose to name a method the same as one of the structs fields:

```Rust
impl Rectangle {
    fn width(&self) -> {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
```

Most often when you're implementing a method with the same name as a field, you are defining a `getter`, in this case, you're returning the value of the field. This is how you can set a field to be private, but the method as public.

#### Automatic Refs

It's worth noting that these are the same, Rust fills in the extra.

```Rust
p1.distance(&p2);
(&p1).distance(&p2);
```

#### Methods with More Params

```Rust
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

#### Associated Functions

All functions in an `impl` block are *associated functions* because they're associated with the type named after the `impl` keyword.

We can define associated functions as anything in the `impl` block that doesn't take `self` as a param - so aren't a method.

We've already used one `String::from` function that's defined on the `String` type.

Associated functions are often used for constructors. These are mostly called `new` - this isn't a special name that's build into the language, but is used by convention.

For example, we could define an associated function called `square` that would have one dimension param, and would return a square `Rectangle`

```Rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
```

To call this we would use the `::` syntax:

```Rust
Rectangle::square(5);
```

#### Multiple `impl` blocks

Each struct is allowed to have multiple `impl` blocks.

```Rust
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
```

## Enums and Pattern Matching

### Defining an Enum

```Rust
enum IpAddrKind {
    V4,
    V6,
}
```

### Enum Values

We can create instances of each of the two `IpAddrKind` like this:

```Rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

As both `V6` and `V4` are of type `IpAddrKind` you can define a function that takes any `IpAddrKind`:

```Rust
fn route(ip_kind: IpAddrKind) {}
```

Add we can call it with either variant:

```Rust
route(IpAddrKind::V4);
route(IpAddrKind::V6);
```

You can give an enum an associated value

```Rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```

Enums can have different associated types with each of their variants:

```Rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
```

### The Option Enum

```Rust
enum Option<T> {
    None,
    Some(T),
}
```

```Rust
let some_number = Some(5);
let some_char = Some('c');

let absent_number: Option<i32> = None;
```

### The Match Control Flow Construct

```Rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

The match contains arms - an arm has a pattern and some code. The first arm here has a pattern of `Coin::Penny` and then the => operator has the code that will run, here that is just the value `1`

You don't typically use curly braces if the code is short.

### Patterns That Bind to Values

```Rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```

You can then use the US State in the arm for that match.

```Rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
```

### The Option<T> match Pattern

Lets say we want a function that takes an Option<i32> and if there's a value, increments it by 1.

```Rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

### Matches are Exhaustive

The patterns must cover all possibilities.

### Catch-All Patterns and the _ Placeholder

Let's say we're playing a game where if the player rolls a `3` they get a fancy new hat, if they roll a `7`, they lose it, and any other number they move by that amount of spaces.

```Rust
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

Note - if you put the catch all arm before other arms, those wont run, and Rust will give you a warning.

Rust also has a `_` to use if you want a catch all pattern, but don't wanna use the variable.

### Concise Control Flow with if let and let else

```Rust
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {max}");
}
```

```Rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {state:?}!");
} else {
    count += 1;
}
```

#### Staying on the happy path with if let else

```Rust
impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            USState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}
```

```Rust
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
```

This kinda feels janky, so we use let else instead

```Rust
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
```
