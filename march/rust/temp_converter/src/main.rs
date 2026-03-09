use std::io;

fn main() {
    'farenheit_or_celcius: loop {
        println!("Would you like to convert to farenheit from celcius? - please enter 1");
        println!("or do you want to convert from celcius to farenheit? - please enter 2");

        let mut conversion_type = String::new();

        io::stdin()
            .read_line(&mut conversion_type)
            .expect("Failed to read line");

        let conversion_type: u8 = match conversion_type.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        let conversion_from: &str;
        let conversion_to: &str;

        if conversion_type == 1 {
            conversion_from = "celcius";
            conversion_to = "farenheit";
        } else if conversion_type == 2 {
            conversion_from = "farenheit";
            conversion_to = "celcius";
        } else {
            println!("That wasn't a valid answer, try again");
            continue;
        }

        loop {
            println!("How many degrees {conversion_from}?");

            let mut input_temp = String::new();

            io::stdin()
                .read_line(&mut input_temp)
                .expect("Failed to read line");

            let input_temp: f32 = match input_temp.trim().parse() {
                Ok(num) => num,
                Err(_) => continue
            };

            let output_temp = if conversion_type == 1 { celcius_to_farenheit(input_temp) } else { farenheit_to_celcius(input_temp) };
            println!("That would be {output_temp} degrees {conversion_to}");
            break 'farenheit_or_celcius;
        };
    };
}

fn farenheit_to_celcius(num: f32) -> f32 {
    (5.0 / 9.0) * (num - 32.0)
}

fn celcius_to_farenheit(num: f32) -> f32 {
    (9.0 / 5.0) * num + 32.0
}
