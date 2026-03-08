# Week 2 notes

## References and Borrowing

A reference is like a pointer in that it's an address we can follow to access the data stored there. That data is owned by something else.

Unlike a pointer, a ref is guaranteed to point to a valid value.

```Rust
fn main() {
    let s1 = String::from("hello");
    
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize { // notice how we take &String, not String
    s.len()
} // here s goes out of scope - but as it doesn't own anything, we can keep using the value behind it
```

As `&s1` does not own the string "hello", it wont be dropped when `&s1` goes out of scope.

We call the action of creating a ref as borrowing. We don't need to give back something in order to give back ownership, as we never had it.

### Mutable References

```Rust
fun main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world")
}
```

You can only have 1 reference if it's mutable. Otherwise there isn't a restriction on number.

```Rust
let mut s = String::from("hello");

{
    let r1 = &mut s;
} // r1 goes out of scope here, so we can make a new ref afterwards with no problems

let r2 = &mut s;
```

```Rust
let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;
println!("{r1} and {r2}");
// var r1 and r2 won't be used again

let r3 = &mut s; // no problem as r1 and r2 aren't relevant
println!("{r3}");
```

### Dangling References

The compiler will guarantee that there won't be any dangling refs - a reference that's out of data and pointing to nothing / something it doesn't intend to.

## Slices

A slice is a kind of ref, so you don't have ownership - lets you reference elements in a collection.

```Rust
fun first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```

### String Slices

A string slice is a ref to a sequence of the elements of a string

```Rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

```Rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return $s[..i];
        }
    }

    &s[..]
}
```

#### String Literals as Slices

```Rust
let s = "Hello, world!";
```

The type of `s` here is `&str` - it's a slice pointing to a particular point in the binary - the code we compiled. It's also why string literals aren't mutable.

#### String Slices as Params

```Rust
fn first_word(s: &String) -> &str {

}
```

A more experienced Rustacean would write the signature below instead because it allows us to use the same function on both `&String` and `&str` values.

```Rust
fn first_word(s: &str) -> &str {

}
```

If we have a string slice, we can pass that in directly. But if we have a string, we can pass a slice or ref of it instead.

#### Other Slices

```Rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```

This slice has type `&32`.
