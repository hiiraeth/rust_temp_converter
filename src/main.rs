use std::io;
use std::io::Write;

fn convert_f_to_c(f: f32) -> f32 { (f - 32.0) * 5.0 / 9.0 }
fn convert_c_to_f(c: f32) -> f32 { (c * 9.0 / 5.0) + 32.0 }

fn prompt(message: &str) -> String {
    let mut input = String::new();
    print!("{}", message);

    io::stdout().flush().unwrap(); // allows print to show before input is taken
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
}

fn parse_temperature(s: &str) -> Option<f32> {
    match s.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}

fn main() {
    let mut my_char: Option<char>;

    let temperature = loop {
        // read in input
        let mut input = prompt("Enter conversion method (F for F to C, vice versa): ");
        input = input.to_uppercase();

        // get first character, validate it
        my_char = input.trim().chars().next();

        match my_char {
            Some('F') | Some('C') => {},
            _ => {
                println!("Invalid input\n");
                continue;
            },
        }

        // get temperature
        let temperature_input = prompt("Enter temperature: ");
        match parse_temperature(&temperature_input) {
            Some(num) => { break num; }
            None => { println!("Invalid input\n"); continue; }
        }
    };

    match my_char {
        Some('F') => println!("\nTemperature in Celsius: {:.1}°", convert_f_to_c(temperature)),
        Some('C') => println!("\nTemperature in Fahrenheit: {:.1}°", convert_c_to_f(temperature)),
        _ => unreachable!(),
    }
}