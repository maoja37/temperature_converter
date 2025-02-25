use std::io;

fn main() {
    loop {
        println!("Do you want to convert from farhenheit to celcius or from celsius to farhenheit");
        println!("To convert from celcius to farhenheit press C, To convert from farhenheit to celsius press F");

        //create a variable to store the wether or not you want to convert celsius or fahrenheit
        let mut conversion_option = String::new();

        //this is used for taking input
        io::stdin()
            .read_line(&mut conversion_option)
            .expect("Failed to read line");

        //you have to convert to lower case in case the user types something
        let conversion_option = conversion_option.trim().to_lowercase();

        // you have to add a as_str() function to this because by default "strings"
        // here return as &str and not String and the variable was declared as a string
        match conversion_option.as_str() {
            "c" => {
                println!("Enter the temperature in celsius");

                //the variable was declared as a floating point for more precision and to avoid truncations
                let temp: f64 = read_temperature();
                println!(
                    "the converted temperature of {temp}°C to Fahrenheit is {}°F",
                    convert_to_fahrenheit(temp)
                );
                // this method returns true except when the user presses 'n' so in that case it returns
                // false and this if statements negates it so it can be equal to true and it can enter the block

                if !ask_to_continue() {
                    break;
                }
            }
            "f" => {
                println!("Enter the temperature in Fahrenheit:");

                let temp: f64 = read_temperature();
                println!(
                    "the converted temperature of {temp}°F to celsius is {}°C",
                    convert_to_celsius(temp)
                );

                if !ask_to_continue() {
                    break;
                }
            }
            _ => {
                println!("Invalid input, try again");
                continue;
            }
        }
    }
}

//first create a function to take in input and convert to celsius
fn convert_to_celsius(farhenheit: f64) -> f64 {
    (farhenheit - 32.0) * 5.0 / 9.0
}

fn convert_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

//to avoid code duplication use this function to read temperature
fn read_temperature() -> f64 {
    loop {
        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");
        match temp.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("invalid number try again"),
        }
    }
}

// this handles wether or not we want to break out of the loop or not
fn ask_to_continue() -> bool {
    println!("Do you want to make another conversion? y/n");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    match choice.trim().to_lowercase().as_str() {
        "y" => true,
        "n" => false,
        _ => {
            println!("Invalid option, I'll assume you want to continue");
            true
        }
    }
}
