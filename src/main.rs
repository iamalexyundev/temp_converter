use std::io;
#[derive(Debug)]
enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}
impl Temperature {
    fn symbol(&self) -> char {
        match self {
            Temperature::Celsius(_) => 'C',
            Temperature::Fahrenheit(_) => 'F',
        }
    }
    fn value(&self) -> f64 {
        match self {
            Temperature::Celsius(val) => *val,
            Temperature::Fahrenheit(val) => *val,
        }
    }
    fn convert(&self) -> Self {
        match self {
            Temperature::Celsius(val) => Temperature::Fahrenheit(*val * (9.0 / 5.0) + 32.0),
            Temperature::Fahrenheit(val) => Temperature::Celsius((*val - 32.0) * 5.0 / 9.0),
        }
    }
}
fn main() {
    println!("Welcome to Temperature converter!");
    println!("Please input the temperature you want to convert from:");

    let temperature_input: f64 = loop {
        match get_user_input().trim().parse() {
            Ok(temp) => break temp,
            Err(_) => println!("Invalid input! Please enter a number!"),
        }
    };

    println!("Please input the unit you want to convert from:");

    let temperature: Temperature = loop {
        match get_user_input().trim().to_lowercase().as_str() {
            "f" => {
                break Temperature::Fahrenheit(temperature_input);
            }
            "c" => {
                break Temperature::Celsius(temperature_input);
            }
            _ => println!("Invalid input! Please enter 'C' or 'F'."),
        }
    };
    println!(
        "You entered: {:.2}°{}",
        temperature.value(),
        temperature.symbol()
    );
    let converted = temperature.convert();
    println!("Converted: {:.2}°{}", converted.value(), converted.symbol())
}

fn get_user_input() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");
    user_input
}
