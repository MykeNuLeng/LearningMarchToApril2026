fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1 // if we put a ; in there, it wont build.
}


// fn main() {
//     let x = five();

//     println!("The value of x is: {x}");
// }

// fn five() -> u8 {
//     5
// }
// statements and expressions

// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };
    
//     println!("The value of y is: {y}");
// }

// functions

// fn main() {
//     println!("Hello, world!");

//     another_function(5);

//     an_extra_function(2, 'm');
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }

// fn an_extra_function(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }