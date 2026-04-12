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

## Packages, Crates, and Modules

A package can contain multiple binary crates, and optionally one library crate.

You can't have two items with the same name in the same scope.

### Packages and Crates

A crate is the smallest amount of code that the Rust compiler considers at a time.

A crate can come in one of two forms:

- A binary crate
    > programs that you can compile to an executable that you can run, each must have a function called `main`
- A library crate
    > define functionality intended to be shared with multiple projects.

A Package is a bundle of one or more crates that provides a set of functionality. A package contains Cargo.toml

#### Control Scope and Privacy with Modules

##### Modules Cheat Sheet

Code within a module is private from its parent modules by default. 

To declare a module as public, use `pub mod` instead of `mod`. To make items within a public module public as well, use `pub` before their declarations.

```Rust
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
```

the `pub mod garden` line tells the compiler to include the code it finds in src/garden.rs

### Paths for Referring to an item in the Module Tree

You can use an absolute path or a relative path.

Both separate identifiers with `::`.

#### Exposing Paths with the `pub` Keyword

```Rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
```

#### Starting Relative Paths with `super`

```Rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

This is similar to using `..` in the start of a relative path in a file system.

### Bringing Paths into Scope with the `use` Keyword

Currently, we have to use the full path of a function to call it - however if you add `use crate::path::to:fn` you can just use the func as if it were defined in this file.

```Rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

Note - I could change the use to have the full path of the function, then I could call the function with just `add_to_waitlist();` however in Rust it is idiomatic to go to the parent of the function, so that it is clear that you are using a function from outside of your scope.

However, when bringing in structs, enums and other items with use, it's idiomatic to call in the full path.

```Rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

However, if you're bringing in two items with the same name, then it's best to call the parents instead:

```Rust
use std::fmt;
use std::io;

fn function() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result {
    // --snip--
}
```

The other way around it is to use the `as` keyword

```Rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult {
    // --snip--
}
```

#### Re-exporting Names with `pub use`

```Rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

Here, `front_of_house` isn't public, so someone using our `lib` wont be able to call anything inside it. However, now we've got a `pub use crate::front_of_house::hosting;` someone can call `use restaurant::hosting::add_to_waitlist()`. We haven't exposed the private `front_of_house` module, but we've allowed someone access to a submodule.

#### Using External Packages

Lets say we want to use `rand` in our project - to do so we'll add `rand = "0.8.5"` to our `Cargo.toml` - which is telling cargo to download the `rand` package and make it available for our project.

Then, whenever we want to bring it into use in our project, we'll add `use rand::Rng;` in that module.

```Rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
```

#### Using Nested Paths to Clean Up `use` Lists

```Rust
use std::cmp::Ordering;
use std::io;
```

Becomes this:

```Rust
use std::{cmp::Ordering, io};
```

Or this:

```Rust
use std::io;
use std::io::Write;
```

Becomes this:

```Rust
use std::io::{self, Write};
```

#### Importing Items with the Glob Operator

If you want to bring in all pub items into scope, you can do this with the `*` / glob operator.

```Rust
use std::collections::*;
```

This can make it hard to keep track of what names you've brought into scope, so be careful.

### Separating Modules into Different Files


In file: src/lib.rs

```Rust
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

In file: src/front_of_house.rs

```Rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

You only need to load a file using `mod` once in the module tree. Other files that want to use it should refer to the file that called the mod statement.

## Common Collections

### Vectors

`Vec<T>`, allows you to store more than one value in a single data structure. Vectors can only store values of the same type.

#### Creating a New Vector

Call `Vec::new();` like so:

```Rust
let v: Vec<i32> = Vec::new();
```

As we didn't add any values to the vector, we had to use type allocation so Rust know what kind of elements to store. 

If you want to implement a vec with values to start, we can use the `vec!` macro.

```Rust
let v = vec![1, 2, 3];
```

#### Updating a Vector

```Rust
let mut v = Vec::new();

v.push(5);
v.push(6);
```

#### Reading Elements of Vectors

```Rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {third}");

let third: Option<&i32> = v.get(2);
match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element"),
}
```

```Rust
let mut v = vec![1, 2, 3, 4, 5];

let first: &i32 = &v[0];

v.push(6);

println!("The first element is: {first}");
```

You get a compiler error here - you've got a ref and then you're going to change the vector.

#### Iterating Over the Values in a Vector

```Rust
let v = vec![1, 2, 3, 4, 5];

for i in &v {
    println!("{i}");
}
```

You can also iterate over mutable refs

```Rust
let mut v = vec![1, 2, 3, 4, 5];

for i in &mut v {
    *i += 50;
}
```

To do this we have to use the `*` dereference operator to get to the value in `i` before we can use the `+=` operator.

#### Using an Enum to Store Multiple Types

```Rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Float(1.11),
    SpreadsheetCell::Text(String::from("Hello")),
];
```

#### Dropping a Vector Drops its Elements

```Rust
{
    let v = vec![1, 2, 3, 4, 5];

    // do stuff with v
} // v goes out of scope
```

### Strings

#### Creating a New String

```Rust
let mut s = String::new();

let data = "initial contents";
let s = data.to_string();

let s = "initial contents".to_string();

let s = String::from("initial contents");
```

#### Updating a String

A string can grow in size and its contents can change, just like the contents of `Vec<T>`.

You can also use the `+` operator or the `format!` macro to concatenate strings.

##### Appending with `push_str` or `push`

```Rust
let mut s = String::from("foo");
s.push_str("bar");
```

The `push_str` method takes a string slice, as we don't necessarily want to take ownership.

The `push` method takes a singe char

```Rust
let mut s = String::from("lo");
s.push('l');
```

##### Concatenating with `+` or `format!`

```Rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");

let s3 = s1 + &s2;
```

Note - s1 is moved here and can no longer be used.

```Rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{s1}-{s2}-{s3}");
```

`format!` takes references, so no ownership is moved.

#### Indexing into Strings

Don't

##### Internal Representation

A `String` is a wrapper over a `Vec<u8>`. 

```Rust
let hello = String::from("Hola");
```

In this case, len will be 4, which means the vector storing this is 4 bytes long. Each of these letters takes 1 bytes when encoded in UTF-8.

```Rust
 let hello = String::from("Здравствуйте");
```

When asked how long this string is, you'd probably say 12, however Rust believes it's 24. That's how many bytes it takes to encode `Здравствуйте` in UTF-8.

##### Bytes, Scalar Values, and Grapheme Clusters

There are three ways to look at strings, as bytes, scalar values and as grapheme clusters.

If we look at the Hindi word “नमस्ते” written in the Devanagari script, it is stored as a vector of u8 values that looks like this:

`[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]`

That’s 18 bytes and is how computers ultimately store this data. If we look at them as Unicode scalar values, which are what Rust’s char type is, those bytes look like this:

`['न', 'म', 'स', '्', 'त', 'े']`

There are six char values here, but the fourth and sixth are not letters: They’re diacritics that don’t make sense on their own. Finally, if we look at them as grapheme clusters, we’d get what a person would call the four letters that make up the Hindi word:

`["न", "म", "स्", "ते"]`

#### Slicing Strings

```Rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```

Here, `s` will be a `&str` that contains the first 4 bytes of `hello`. Which means that `s` will be `Зд`. (З - 2 byte, д - 2 bytes)

If you try only to slice part of a chars bytes, Rust will panic at runtime. Be careful creating string slices with ranges.

#### Iterating Over Strings

Best way is to specify if you want to iterate over chars or bytes

```Rust
for c in "Зд".chars() {
    println!("{c}"); // will print 3 д
}
```

```Rust
for c in "Зд".bytes() {
    println!("P={c}"); // 208 151 208 180
}
```

If you want to get grapheme clusters from strings, you will need to find a crate to do so - it's not offered by the standard lib.

### Hash Maps

The type `HashMap<k, v>` stores a mapping of keys against values.

#### Creating a New Hash Map

One way is to use `new` and then add elements with `insert`.

```Rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

Note, we need to `use` the hashmap from the std lib before using it.

Just like vectors, hashmaps store their data on the heap. All keys must have the same type, and all values must have the same type.

#### Accessing Values in a Hash Map

```Rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_op(0);
```

Here `score` will have the value that's associated with the Blue team, and the result will be `10`. 

The `get()` method returns an `Option<&i32>`

You can iterate over each key value pair

```Rust
for (key, value) in &scores {
    println!("{key}: {value}");
}
```

#### Managing Ownership in Hash Maps

For types that implement the `Copy` trait, like `i32`, the values are copied into the hash map. For owned values like `String`, the values will be moved, and the map will become the owner.

```Rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);

// field_name and field_value are invalid at this point.
```

#### Updating a Hash Map

##### Overwriting a Value

```Rust
use std:collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{scores:?}"); // {"Blue": 25}
```

##### Adding a Key and Value Only if a Key isn't Present

```Rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{scores:?}"); // {"Yellow": 50, "Blue": 10}
```

##### Updating a Value Based on the Old Value

```Rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{map:?}"); // {"world": 2, "hello": 1, "wonderful": 1}
```

## Errors

### Panic!

This is where my app crashes because I tried to access an element in a vector that doesn't exist. Rust backtraces are pretty useful.

Can intentionally panic! like this:

```Rust
fn main() {
    panic!("Crash and Burn!");
}
```

### Recoverable Errors

The result enum has two variants:

```Rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

```Rust
use std::io::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    }
}
```

Sometimes you want to keep the flow going, even when there's an error

```Rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}"),
            }
        },
    };
}
```

#### Shortcuts for Panic on Error

Calling `unwrap` on a result will either return the ok value, or cause the program to panic.

```Rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}
```

Using `expect` instead of `unwrap` and providing good error messages can help to convey my intent

```Rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
```

#### Propagating Errors

You can return an error to the calling code, so that it can decide what to do.

```Rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(error) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```

#### The ? Shortcut

```Rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username);
}
```

The error type is converted into the error type in the signature of the function. So if something goes wrong, the error returned will be `io::Error`.

By chaining these methods, we can make it even more terse.

```Rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
```

We can make it even more terse by using `fs::read_to_string`

```Rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

#### Where to Use the ? Operator

It can only be used in functions who's return type is compatible with the value the `?` is used on.

```Rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")?; // wont work, return type in main is () -> so what can the error return on failure?
}
```

We can only use `?` on a function with a return type of `Result`, `Option`, or `FromResidual`

```Rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```

You can change the return type of `main` so that you can use `?`

```Rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
```

### To Panic! or Not to Panic!

When there's no way to recover - `panic!`.
When an error is an expected outcome that we can accommodate for, don't `panic!`.

#### When You Have More Information Than the Compiler

If you know from some logic earlier in your program that there is no way for an error to occur, then it's a good idea to use `expect` and document the reason you'll never have an `Err` variant.

```Rust
use std::net::IpAddr;

let home: IpAddr = "127.0.0.1"
    .parse()
    .expect("hardcoded IP address should be valid");
```

#### Custom Types for Validation

```Rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```

## Writing Automated Tests

### How to Write Tests

Tests are Rust functions that verify that non test code is working as expected.

The bodies of these tests typically do 3 things:

- Set up any needed data or state
- Run the code you want to test
- Assert that the results are what you expect

#### Structuring Test Functions

A Test in Rust is a function with the `test` attribute. Attributes are metadata about pieces of Rust code - one example is `derive`.

To turn a function into a test function - write `#[test]` on the line before.

Run the tests with `cargo test`

#### Checking Tests with Assert

The `assert!` macro defined by the standard lib will panic if the condition doesn't equate to `true`.

#### Testing Equality with `assert_eq` and `assert_ne`

`assert_ne!` is useful if you know that some value will change in a function, but not entirely sure exactly to what.

#### Using `Result<T, E>` in Tests

This enables you to use the `?` operator in the body of the test.

### Controlling how Tests are Run

By default `cargo test` will run all the tests in parallel.

#### Running Tests in Parallel or Consecutively

If you don't want to run the tests in parallel, you can run them consecutively by using the command `cargo test -- --test-threads=1`

#### Showing Function Output

If you fail a test with `assert_eq` you will see the values for both left and right hand sides.

If you want to see the values even if you pass, use `cargo test -- --show-output`

#### Running a Subset of Tests by Name

##### Running Single Tests

You can pass the name of any test to `cargo test` to have only that test run.

##### Filtering to Run Multiple Tests

If you have two tests that both have `add` in the functions, you could run `cargo test add` to have both of them run.

#### Ignoring Tests Unless Specifically Requested

```Rust
#[test]
#[ignore]
fn ignore_this_test() {
    // --snip--
}
```

### Test Organization

Unit tests are kept in the files with the functions that they are testing.

Any module with the `#[cfg(test)]` annotation will mean the compiler will ignore it.

#### Integration Tests

Integration tests are held within the Tests directory. Which is held at the top level of the file, along with the Cargo.lock and Cargo.toml

```Rust
use adder::add_two;

#[test]
fn it_adds_two() {
    let result = add_two(2);

    assert_eq!(result, 4);
}
```

## Iterators and Closures

### Closures

#### Inferring and Annotating Closure Types

Closures don't usually require you to annotate the types of the arguments or return values. You can, but it's a bit odd.

```Rust
let expensive_closure = |num: u32| -> u32 {
    num * 2
}
```

here's a function and a closure that add x + 1.

```Rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

The compiler will infer a type for a closure, and then won't be able to reassign that dynamically.

```Rust
let closure = |x| x;

let y = closure(string::from("hi")); // y = "hi"
let x = closure(1); // compiler error, as closure has been assigned |x: String| -> String { x };
```

#### Capturing References or Moving Ownership

Closures can capture values in three ways:

- borrowing immutably
- borrowing mutably
- taking ownership

```Rust
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}"); // 1, 2, 3

    let only_borrows = || println!("from closure: {list:?}"); // 1, 2, 3

    println!("Before calling closure: {list:?}"); // 1, 2, 3
    only_borrows();
    println!("After calling closure: {list:?}"); // 1, 2, 3
}
```

```Rust
fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}"); // 1, 2, 3

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {list:?}"); 1, 2, 3, 7
}
```

When `borrows_mutably` is defined, it captures a mutable reference to `list` - so it wouldn't be possible to println! in between the closure definition and it being called.

If you want to force a closure to take ownership of the values it is passed, you can use the `move` keyword before the param list. This is mostly useful if you're sending the closure to another thread.

```Rust
fn main() {
    let mut list = vec![1, 2, 3];
    println!("before calling closure: {list:?}");

    thread::spawn(move || println!("from thread: {list:?}"))
        .join()
        .unwrap();
}
```

Here we spawn a new thread and give the thread a closure to run. Even though the closure body only needs an immutable reference to `list` we still need to move ownership to that thread.

#### Moving Captured Values Out of Closures

A closure body can do any of the following:

- Move a captured value out of the closure
- Mutate the captured value
- Neither mutate nor move
- Capture nothing to begin with

Closures will automatically implement 1, 2 or 3 of the following traits additively depending on how they interact with these values:

- `FnOnce` Applies to closures that can be called once - Which applies to all of them. A closure that moves values out of its body will only implement `FnOnce` and none of the others, because that means it can only be called once.
- `FnMut` Applies to closures that wont move captured values, but might mutate them - these can be called more than once.
- `Fn` Applies to closures that neither move nor mutate captured values, and also to closures that don't capture anything. These closures can be called more than once without mutating their environment. This is important if you're calling this closure multiple times concurrently.

Let's look at `unwrap_or_else`

```Rust
impl<T> Option<T> {
    pub fn unwrap_or_else(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

The trait bound on `F` is `FnOnce() -> T` meaning that `F` must be able to be called once, take no arguments and return a `T`.

```Rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 5, height: 10 },
        Rectangle { width: 7, height: 4 },
    ];

    let sort_by_key(|r| r.width);
    println!("{list:#?}");
}
```

The reason `sort_by_key` is defined to take a `FnMut` closure is because the closure will need to be run multiple times to compare each element.

In contrast, if you used a closure that moves an item out of its body, then it wouldn't compile

```Rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 5, height: 10 },
        Rectangle { width: 7, height: 4 },
    ];

    let mut sort_operations = vec![];
    let value = String::from("closure called");

    let sort_by_key(|r| {
        sort_operations.push(value);
        r.width
    });
    println!("{list:#?}");
}
```

This closure can only be called once, as it takes ownership of `value` and shoves it into `sort_operations`, calling it a second time would cause issues, so the compiler won't be happy with this code at all.

### Iterators

#### Processing a Series of Items with Iterators

In Rust, iterators are lazy, meaning they have no effect until you call a method that consumes them.

```Rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got: {val:?}");
}
```

#### The Iterator Trait and the Next Method

As defined in a the standard lib:

```Rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>
}
```

This definition uses some new syntax - `type Item` and `Self::Item`, which are defining an associated type with that trait. - TLDR this means that implementing the iterator trait requires you to define an item type and that the item type is used in the return type of the next method.

```Rust
#[test]
fn iterator_demo() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
```

Note, we need to make `v1_iter` mutable, as each time we call next() it changes the internal state.

If you want to take an iter that takes ownership of v1 and returns owned values, we can call `into_iter` instead.

If you want to iterate over mutable references instead, we can call `iter_mut`

#### Methods That Consume the Iterator

```Rust
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter().sum();

    assert_eq(total, 6);
}
```

We can't then use v1_iter after the sum as `sum` takes ownership.

#### Methods That Produce Other Iterators

Iterator adapters are defined on the iterator trait that don't consume the iterator. Instead they produce different iterators by changing some aspect of the original.

For example - `map`

#### Closures That Capture Their Environment

Many iterator adapters take closures as arguments. Commonly these closures also capture their environment.

## Cargo and Crates

### Customizing Builds with Release Profiles

Cargo has default settings for a `--release` vs a `dev` build. In the `Cargo.toml` there is a setting where you can change the optimization level of the build, 0 being no opt, 3 being max.
By default, the settings will look like this:

```Cargo.toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

The higher optimization comes at the cost of longer compile times.

### Publishing a Crate to Crates.io

#### Documentation Comments

Documentations comments use `///` rather than the standard `//` and also support markdown formatting.

## Smart Pointers

The most common kind of pointer in Rust is a reference. They don't have any special capabilities other than referring to data, and they have no overhead.

Smart pointers are data structures that act like a pointer but have additional metadata and capabilities.

While references borrow data, smart pointers in many cases own the data they point to.

They implement the `Deref` and `Drop` traits. The `Deref` trait allows a smart pointer to behave like a reference. The `Drop` trait allows you to customize the code that's run when an instance of the smart pointer goes out of scope.

The most common are:

- `Box<T>` - for allocating values on the heap
- `Rc<T>` - allows multiple ownership
- `Ref<T>` - enforces borrowing rules at runtime rather than compile time.

### Using `Box<T>` to Point to Data on the Heap

The most straightforward smart pointer is a box. Boxes allow you to store data on the heap rather than the stack. What remains on the stack is a pointer to the heap data.

They are used when:

- You have a type who's size can't be known at compile time.
- You have a large amount of data, and want to transfer ownership, but don't want it to be copied
- You want to own a value, and only care that it's a type that implements a particular trait, rather than a specific type.

#### Storing Data on the Heap

This is how you store an `i32` on the heap:

```Rust
fn main() {
    let b = Box::new(5);
    println!("b = {b}");
}
```

#### Enabling Recursive Types with Boxes

A value of a recursive type can have another value of the same type as part of itself. This poses an issue as Rust needs to know at compile time how much space it will take up.

However, the nesting of values of recursive types could theoretically continue infinitely - so no dice.

##### Understanding the Cons List

A *cons list* is a data structure that comes from Lisp, and is made up of nested pairs, and is the Lisp version of a linked list.

Here's the pseudocode representation of a cons list containing list `1, 2, 3`.

```Lisp
(1, (2, (3, Nil)))
```

```Rust
enum List {
    Cons(i32, List), // wont compile
    Nil,
}
```

Using the `List` type to store the list `1, 2, 3` would look like the code below:

```Rust
use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil))); // wont compile
}
```

##### Getting a Recursive Type with a Known Size

Instead of storing the value directly, we should change the data structure to store the value indirectly by storing a pointer instead.

Because `Box<T>` is a pointer, Rust always knows how much space `Box<T>` needs - A pointer's size doesn't change based on how much data it is pointing to.

This means we can put `Box<T>` inside the `Cons` variant, instead of another List value directly. The `Box<T>` will point to the next `List` value that will be on the heap. Conceptually we still have a list of one value containing other lists, but this way is more like placing data next to each other rather than nesting.

```Rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))); 
}
```

The `Cons` variant now needs to be able to store the `i32` and a pointer. The compiler knows how big those are, so we're all good.

### Treating Smart Pointers Like Regular References

Implementing the `Deref` trait allows you to customize the behavior of the *dereference operator* `*`.

#### Following the Reference to the Value

Here we create a ref to an `i32` and then use `*` to follow the ref to the value

```Rust
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

if we tried to do `assert_eq!(5, y)` we'd get a compiler error saying it can't compare `{integer} with &{integer}`.

#### Using `Box<T>` Like a Reference

```Rust
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, 5);
    assert_eq!(5, *y);
}
```

#### Defining Our Own Smart Pointer

Lets hand roll a wrapper similar to `Box<T>` to experience how smart pointers are different from refs. Note - our version will store data on the stack, so not perfectly analogous.

```Rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(T)
    }
}
```

```Rust
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // won't compile - we haven't told the compiler how to dereference yet.
}
```

#### Implementing the `Deref` Trait

```Rust
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T; // defines an associated type for the deref to use.

    fn deref(&self) -> &Self::Target {
        &self.0 // returns a ref to the value we want to access when using *
    }
}
```

Now the above code would compile.

#### Using Deref Coercion in Functions and Methods

*Deref coercion* converts a ref of one type that implements `Deref` to a ref of another type. For example, this can convert a `&String` to `&str` - this is because `String` implements the `Deref` trait such that it returns a `&str`.

```Rust
fn hello(name: &str) {
    println!("Hello, {name}!"); // we can call this with a string slice as an arg like hello("Rust"); we can also call it with a ref to MyBox<String>
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // this works due to deref coercion
}
```

`&m` is a reference to the `MyBox<String>` value. Because we implemented the deref trait on `MyBox<T>` rust can convert `&MyBox<String>` into `&String` by calling `deref`. The standard lib provides a deref for `String` that returns a slice, and so Rust calls `deref` again and converts it into `&str`, which matches the functions definition.

If Rust didn't do this, we'd have to do this instead:

```Rust
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]); // *m gives a String, the & then gives a &String, and [..] takes a slice, so you get &str
}
```

When the `Deref` trait is defined for the types involved, Rust will use `Deref::deref` as many times as necessary to get a reference to match the parameters type.

#### Handling Deref Coercion with Mutable References

Similar to how you use the `Deref` trait to override the `*` operator on immutable refs, you use the `DerefMut` to override the `*` operator on mutable refs.

Rust does deref coercion when it finds types and trait implementations in 3 cases:

- From `&T` to `&U` when `T: Deref<Target=U>`
- From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
- From `&mut T` to `&U` when `T: Deref<Target=U>`

So you can coerce a mutable ref into an immutable one, but not the other way around.

This is because of the borrowing rules - if you have a mutable ref, that must be the only one to that data, so if you're getting mutable references from deref coercion, you can't keep track of it anymore.

### Running Code on Cleanup with the `Drop` Trait

`Drop` lets you customize what happens when a value is about to go out of scope. For example, when a `Box<T>` is dropped, the space on the heap that it's pointing to is deallocated.

As Rust automatically drops values as they go out of scope, you don't need to be worried about manually dropping values.

The `Drop` trait requires you define a method named `drop` that takes a mutable ref to `self`.

```Rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created");
}
```

Variables are dropped in reverse order of their creation - so you'd get a print from d before c.

If you want a value dropped before it goes out of scope, you have to use `std::mem::drop`.

```Rust
use std::mem::drop;

fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };

    println!("CustomSmartPointer created");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main!");
}
```

### `Rc<T>`, the Reference-Counted Smart Pointer

You have to enable multiple ownership in Rust by using the type `Rc<T>`, which is an abbreviation for reference counting. The type keeps track of the number of references to determine if it's still in use. If that reaches 0, then it will be cleaned up.

Imagine it as a tv in a family room, when the first person comes in, it is turned on, and it is only turned off as the last person leaves.

#### Sharing Data

We'll create two lists, `b` and `c` that both share list `a`.

```Rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a)); // won't compile
}
```

```Rust
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons, Rc::New(Cons(10, Rc::new(Nil))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
```

The use of `Rc::clone` doesn't create a deep copy of all the data. It only increments the reference count of `a`.

#### Cloning to Increase the Reference Count

```Rust
// --snip--

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a)); // 1

    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a)); // 2

    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a)); // 3
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a)); // 2
}
```

### `RefCell<T>` and the Interior Mutability Pattern

*Interior mutability* is a patttern in Rust, where you can mutate data even when there are immutable references to the data.

The pattern uses `unsafe` code inside a data structure that bends Rust's usual rules.

Unsafe code lets the compiler know that we're doing the checking manually rather than relying on it.

#### Enforcing Borring Rules at Runtime

Unlike `Rc<T>`, `RefCell<T>` represents single ownership over the data it holds.

Because some analysis is impossible, Rust's compiler might reject some code that would be perfectly possible. You can use `RefCell<T>` in these scenarios.

`RefCell<T>` is only used in single threaded scenarios.

#### Using Interior Mutability

```Rust
fn main() {
    let x = 5;
    let y = &mut x; // wont compile x isn't mutable
}
```

##### Testing with Mock Objects

```Rust
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    user super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}
```

This won't compile as the mock messenger is not mutable in send, but we're trying to push to the vector it contains.

So we'll use `RefCell<T>` instead

```Rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // --snip--

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
```

##### Tracking Borrows at Runtime

```Rust
impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        let mut one_borrow = self.sent_messages.borrow_mut();
        let mut two_borrow = self.sent_messages.borrow_mut();

        one_borrow.push(String::from(message));
        two_borrow.push(String::from(message)); // will compile, but will panic! we have created two mutable references...
    }
}
```

##### Allowing Multiple Owners of Mutable Data

A common way to use `RefCell<T>` is in combination with `Rc<T>`. By combining the two, you get something that can have multiple owners, and can be mutated.

```Rust
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::cell::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}"); // 15, nil
    println!("b after = {b:?}"); // 3, 15, nil
    println!("c after = {c:?}"); // 4, 15, nil
}
```

### Reference Cycles Can Leak Memory

Rust's memory safety guarantees make it difficult but not impossible, to accidentally create memory that is never cleaned up.

#### Creating a Reference Cycle

```Rust
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::cell::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    println!("a next item = {:?}", a.tail()); // will cause a stack overflow
}
```

#### Preventing Reference Cycles Using `Weak<T>`

You can create a weak ref by calling `Rc::downgrade` and passing a ref to the `Rc<T>`. Strong refs are how you share ownership, weak refs don't express ownership, and their count is independent to when the var is cleaned up.

If you want to then use the downgraded ref, you need to upgrade it again and check it exists, and the upgrade function returns an option.

##### Creating a Tree Data Structure

To start we'll build a tree with nodes that know about their child nodes.

```Rust
use std::cell::{RefCell, Rc};

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
}
```

##### Adding a Reference from a Child to its Parent

```Rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // None

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); //  Some(...)
}
```

##### Visualizing Changes to `strong_count` and `weak_count`

```Rust
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong: {}, weak: {}",
        Rc::strong_count(&leaf), // 1
        Rc::weak_count(&leaf), // 0
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong: {}, weak: {}",
            Rc::strong_count(&branch), // 1
            Rc::weak_count(&branch), // 1
        );

        println!(
            "leaf strong: {}, weak: {}",
            Rc::strong_count(&leaf), // 2
            Rc::weak_count(&leaf), // 0
        );
    }

    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong: {}, weak: {}",
        Rc::strong_count(&leaf), // 1
        Rc::weak_count(&branch), // 0
    );
}
```
