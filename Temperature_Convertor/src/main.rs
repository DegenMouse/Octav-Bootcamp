use std::io::{self, Write};
use std::str::FromStr;

fn main() {
    menu()
}

fn menu() {
    loop {
        println!("\n----------------MENU----------------");
        println!("[1] to convert celsius to fahrenheit");
        println!("[2] to convert fahrenheit to celsius");
        println!("[0] to exit\n");
        let option: i32 = get_input("Please enter an option ");
        match option {
            1 => convert_to_f(),
            2 => convert_to_c(),
            0 => break,
            _ => println!("Invalid option"),
        }
    }
}

fn get_input<T: FromStr>(text: &str) -> T {
    print!("{}", text);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
        .trim()
        .parse::<T>()
        .unwrap_or_else(|_| panic!("Failed to parse input to the desired type"))
}

fn convert_to_f() {
    let celsius: f32 = get_input("Please enter the temperature in celsius: ");
    println!(
        "The converted temperature is {}°F",
        (celsius * 9.0 / 5.0) + 32.0
    );
}

fn convert_to_c() {
    let fahrenheit: f32 = get_input("Please enter the temperature in fahrenheit: ");
    println!(
        "The converted temperature is {}°C",
        (fahrenheit - 32.0) * 5.0 / 9.0
    );
}

#[cfg(test)]
mod tests {
}
