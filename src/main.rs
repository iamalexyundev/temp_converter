use std::io;
fn main() {
    println!("Welcome to Temperature converter!");
    println!("Please input the temperature you want to convert from:");

    let temperature: f64 = loop {
        match get_user_input().trim().parse() {
            Ok(temp) => break temp,
            Err(_) => println!("Invalid input! Please enter a number!"),
        }
    };

    println!("Please input the unit you want to convert from:");

    let unit = loop {
        match get_user_input().trim().to_lowercase().as_str() {
            "f" => {
                println!("Converting {temperature} from F to C");
                break "F";
            }
            "c" => {
                println!("Converting {temperature} from C to F");
                break "C";
            }
            _ => println!("Invalid input! Please enter 'C' or 'F'."),
        }
    };
    println!("You entered: {:.2}°{}", temperature, unit);
    let converted = convert(temperature, unit);
    println!(
        "{temperature:.2}°{unit} is {:.2}°{}",
        converted.0, converted.1
    )
}

fn get_user_input() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");
    user_input
}

fn convert(temperature: f64, symbol: &str) -> (f64, &str) {
    match symbol {
        "F" => ((temperature - 32.0) * 5.0 / 9.0, "C"),
        "C" => (temperature * (9.0 / 5.0) + 32.0, "F"),
        _ => {
            println!("Something went wrong!");
            (temperature, symbol)
        }
    }
}
