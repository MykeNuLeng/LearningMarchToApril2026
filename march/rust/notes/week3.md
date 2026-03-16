# Week 3 Rust Notes

## Generics, Traits, and Lifetimes

### Generics

When we use a type parameter in the function signature, we must declare it before we use it - this is in the `<>`.

```Rust
fn largest<T>(list: &[T]) -> &T { }
```

Now, because we want to compare different values of `T`, we need to constrain it to things that can be compared. To do this, we can use `std::cmp::PartialOrd`.

So we get this:

```Rust
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T { }
```

#### Struct Definitions

We can also use generics in structs by using the angle bracket syntax. 

```Rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

Note: because we used only one type T, if we tried to create a Point struct with x as an int and y as a float, it wouldn't compile.

```Rust
struct Point<T, U> {
    x: T,
    y: U,
}
```

Now, we can have both T and U being the same type, or different.

#### Enum Definitions

```Rust
enum Option<T> {
    Some<T>,
    None,
}
```

Now this enum is like an optional in swift - Some holds a value of type T, and None doesn't hold any value.

Similarly, the result enum works by holding two generics

```Rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

#### Method Definitions

We can implement methods on structs and enumbs and use generic types in their definitions too.

```Rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

Here we've defined a method called `x` on `Point<T>` that returns a reference to the data in `x`.

Note - we declared `T` just after the `impl` keyword.

If we wanted to create a method on `Point<T>` that only worked on `f32`s, we could do that like this:

```Rust
impl Point<i32> { }
```

If you want to use generics that are relevant to the struct, you must define it next to impl - otherwise if it's just relevant to the method, you must define it there.

#### Performance

Generics wont make your code run any slower than with concrete types. Rust does this by assigning the Generics concrete types at compile time.

### Traits

A trait defines the functionality a particular type has and can share with other types. It's a way to define behavior in an abstract way.

These are sometimes called interfaces in other languages.

#### Implementing a trait on a type

Implementing a trait on a type is similar to implementing regular methods.

after `impl` we put the trait name we want to implement, then use the `for` keyword, then specify the name of the type we want to implement the trait for.

##### Default implementations

```Rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

impl Summary for NewsArticle {}
```

Now we can use the default implementation of summarize on an instance of the NewsArticle struct.

#### Using Traits as Parameters

```Rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

Now, instead of taking a concrete type as a parameter, we specify the `impl` keyword and the trait that we need the type to adhere to.

##### Trait Bound Syntax

The `impl Trait` syntax works, but is shorthand for the longer trait bound syntax.

```Rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

###### Multiple Trait Bounds with the + Syntax

Say we wanted notify to use display formatting as well as summarize on item.

```Rust
pub fn notify(item: &(impl Summary + Display)) {

}

pub fn notify<T: Summary + Display>(item: &T) {

}
```

###### Clearer Trait Bounds with `where` clauses

Each generic has its own trait bound, so functions with multiple generics get messy quick. So instead you can use a `where` clause after a function signature

So this: 

```Rust
fn some_function<T: Display + Clone, U: Clone, Debug>(t: &T, u: &U) -> i32 { }
```

Becomes this:

```Rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{ 

}
```

This way the signature becomes less cluttered, and easier to read.

#### Returning Types that Implement Traits

We can also use the `impl Trait` syntax in the return of a signature.

```Rust
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people"
        ),
        reply: false,
        repost: false,
    }
}
```

This will be more relevant when we cover closures and iterators.

However, it's important this only works if you can return one type. If your code can return either a bool or string for example, the compiler will loudly complain.

##### Using Trait Bounds to Conditionally Implement Methods

blanket implementation is like saying, anything that adheres to x also adheres to y - say we had a trait called toes, you could then say anything that is a toe, also adheres to body parts

```Rust
impl<T: Display> ToString for T {
    //
}
```

We are saying here if T adheres to display, then it will also adhere to ToString

This means that as integers have adhere to display, we can do this:

```Rust
let s = 3.to_string();
```

### Lifetimes

Lifetimes are another kind of generic. Rather than assuring that a type has a particular trait that we want, it is assuring that a reference is valid for as long as it is needed.

Every ref in Rust has a lifetime, which is the scope in which it is valid. Most of the time, they are implicit and inferred.

Similarly, we only need to annotate a type when another is possible. With lifetimes, we need to clarify when it's ambiguous.

#### Dangling References

Lifetimes mostly exist to prevent this. Essentially it is a pointer aiming where its data used to be.

```Rust
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {r}");
}
```

The compiler won't be happy here - you're trying to use a reference for some data that has been dropped.

#### The Borrow Checker

Rust compiler has a borrow checker that compares scopes to determine if all borrows are valid.

#### Generic Lifetimes in Functions

```Rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("the longest string is: {result}");
}
```

The function longest compares string slices and returns the longest. We want this to take slices as they are references, and don't want longest to take ownership of them.

This won't compile:

```Rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
```

The compiler is unhappy because it can't tell if the return will be a borrowed value from x or y -> so you need to specify at this point, or use a generic.

#### Lifetime Annotation Syntax

Lifetime annotations don't change how long any of the references will live.

They describe the relationship of the references to each other.

The names of lifetime params must start with an apostrophe `'` - most people use the name `'a` for the first lifetime annotation.

We place the lifetime annotation after the `&`of a reference.

#### In Function Signatures

Define the params inside angle brackets.

We want the signature to express the following constraint -> The returned reference will be valid as long as the params. So, we'll add the `'a` to both the references and the return.

```Rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {

}
```

This is saying to the borrow checker - don't worry about the return - it will last as long as both x and y. It will then check if that's correct, and loudly complain if not.

```Rust
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }
}
```

The borrow checker is happy here! `string1` lasts until the end of main, `string2` lasts until the end of the inner block, and the result lasts until the end of the inner block, so there's no way that something can go wrong.

```Rust
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
    }

    println!("The longest string is {result}");
}
```

Now here, `string1` still has the same lifetime and so does `string2`, but the result is needed outside of the block, which is longer than `string2` is alive. We promised that the result wouldn't be needed after either of them had been dropped.

The borrow checker will loudly complain.

#### Relationships

The way you need to specify lifetime parameters depends on what the function is doing, for example, if your function always returned the first string, then you won't need to annotate the second, because who cares if it's valid or not??

```Rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```

Now, you may think, why won't this compile? It's because you're borrowing from result, which is owned within the function. As soon as the function ends, result is dropped, and the value that you're renting from has disappeared.

#### In Struct Definitions

If you're defining a struct with references, you need to use lifetime annotations

```Rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();

    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

This means that an instance of Important excerpt cannot outlive the string it's taking a slice from.

#### Lifetime Elision

You've seen this function compile without lifetime annotations:

```Rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..1];
        }
    }

    &s[..]
}
```

In early versions of Rust, this wouldn't have compiled - every reference needed an explicit lifetime. At that time, the signature would have been:

```Rust
fn first_word<'a>(s: &'a str) -> &'a str { }
```

It is possible in the future that more deterministic lifetimes will be added to the compiler.

lifetimes on function or method params are called input lifetimes and those on return values are called output lifetimes.

There are three rules that the compiler checks when there aren't explicit lifetimes, these rules apply to `fn` and `impl` blocks:

1. The compiler assigns a lifetime param to each param that's a ref. For example `fn foo<'a>(x: &'a i32);`
2. If there is exactly one input lifetime param, that lifetime is assigned to all the output lifetime params - `fn foo<'a>(x: &'a i32) -> &'a i32`.
3. If there are multiple input lifetime params, but one is `&self` or `&mut self`, the lifetime of `self` is assigned to all output lifetimes.

This signature starts without any lifetimes associated with the references:

```Rust
fn first_word(s: &str) -> &str { }
```

Here the compiler applies the first rule where each param gets its own lifetime:

```Rust
fn first_word<'a>(s: &'a str) -> &str { }
```

The second rule then applies because there is exactly one input lifetime:

```Rust
fn first_word<'a>(s: &'a str) -> &'a str { }
```

Now all references in this function have lifetimes associated with them - the compiler is at peace.

Lets look at another example, our `longest` function:

```Rust
fn longest(s: &str, y: &str) -> &str { }
```

You can see that rule 2 doesn't apply here as there are multiple input lifetime params. Rule 3 also doesn't apply as there is no reference to self.

Rule 1 works:

```Rust
fn longest<'a>(s: &'a str, y: &'a str) -> &str { }
```

However we still haven't figure out what the lifetime of the return reference will be, so the compiler will calmly ask the user what it means.

##### In Method Definitions

Lifetime names for struct fields always need to be declared after the `impl` keyword and then used after the struct's name, because those lifetimes are part of the struct's type.

Lifetime elision rules often make it so that lifetime annotations aren't necessary in method signatures.

```Rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

Because of the first elision rule, the compiler can figure out how to assign a lifetime to the param.

Here is an example of the 3rd rule in play:

```Rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}
```

First of all the compiler will apply the first rule to give `&self` and `announcement` a lifetime.

The compiler will then apply the 3rd rule, which will give the output the same lifetime as &self. This means that all references are accounted for, and the compiler is at peace.

##### The Static Lifetime

One special lifetime is the `'static` lifetime, which denotes that the associated reference *can* live for the entire duration of the program. All string literals have a `'static` lifetime which we can annotate like this:

```Rust
let s: &'static str = "I have a a static lifetime.";
```

The text of this string is stored directly in the program's bin, which is always available.

You might see error messages that suggest using `'static`, but before applying this think about whether the ref actually lives for the entire duration of the program, and if you want it to.

Most of the time this error suggestion comes about from creating a dangling reference or a mismatch of the available lifetimes.

### Generic Type Params, Trait Bounds and Lifetimes

Let's quickly look at the syntax of specifying generic type params, trait bounds and lifetimes in one function.

```Rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}
```

`ann` needs to have trait `T` which adheres to `Display` because we are printing it using `{}`.
