use std::io::{stdin};

// A simple temperature convertor app that takes a temperature value from the user and converts between Fahrenheit and Celcius

fn main() {
    let mut temperature = String::new();
    let mut converter_option = String::new();
    println!("Enter temperature");
    stdin().read_line(&mut temperature).expect("Failed to read temperature value");
    let temperature: f32 = temperature
        .trim()
        .parse()
        .expect("Temperature value entered was not a number");
    println!("Convert to: \n 1. Fahrenheit to Celcius \n 2. Celcius to Fahrenheit");

    stdin().read_line(&mut converter_option).expect("Failed to read convertion option");
    let converter_option: char = converter_option.trim().chars().next().unwrap();

    match converter_option {
        '1' => println!("{}°F from Fahrenheit to Celcius is {}°C", temperature, (temperature - 32.0) * 0.555556),
        '2' => println!("{}°C from Celcius to Fahrenheit is {}°F", temperature, (temperature - 1.8) + 32.0),
        _ => println!("Wrong option! Try again please...")
    }
}
