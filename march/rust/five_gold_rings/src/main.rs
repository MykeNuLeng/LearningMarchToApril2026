const ORDINALS: [&str; 12] = [
        "first", "second", "third", "fourth",
        "fifth", "sixth", "seventh", "eighth",
        "ninth", "tenth", "eleventh", "twelfth",
    ];
    const GIFTS: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves and",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

fn main() {
    for number in 0..12 {
        println!("On the {} day of Christmas my true love sent to me", ORDINALS[number]);

        for gift in GIFTS[..=number].iter().rev() {
            println!("{}", gift);
        }
    }
}
