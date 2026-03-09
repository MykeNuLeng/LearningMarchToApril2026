use std::io;

fn main() {
    loop {
        println!("I can find the nth fibonacci number");
        println!("I can prove it, give me a value of n");

        let mut n = String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");

        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        if n < 1 {
            println!("That is not a valid n for fibonacci");
            continue;
        }

        let mut cur = 1;
        let mut temp: u32;
        let mut prev = 0;
        for _number in 1..n {
            temp = cur;
            cur = cur + prev;
            prev = temp;
        }
        println!("n is: {cur}");
        break;
    }
}
