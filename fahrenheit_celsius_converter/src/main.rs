use std::io;

fn fahrenheit_to_celsius(d: i32) -> f64 {
    ((d - 32) as f64) * 5.0 / 9.0
}

fn celsius_to_fahnreheit(d: i32) -> f64 {
    (d as f64) * 1.8 + 32.0
}

fn main() {
    loop {
        println!("Do you want to convert to Celsius or Fahrenheit? Input C or F");

        let mut unit = String::new();
        io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read line..");
        let unit = match unit.trim().to_ascii_uppercase().as_str() {
            "C" => String::from("Celsius"),
            "F" => String::from("Celsius"),
            _ => continue,
        };
        println!("You have picked {}", unit);
        loop {
            let mut temperature = String::new();

            match unit.as_str() {
                "Celsius" => println!("Please input the temperature in Fahrenheit"),
                "Fahrenheit" => println!("Please input the temperature in Celsius"),
                _ => println!("Something went wrong.."),
            }

            io::stdin()
                .read_line(&mut temperature)
                .expect("Failed to read line..");

            let temperature: i32 = match temperature.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            match unit.as_str() {
                "Celsius" => {
                    println!(
                        "{} Fahrenheit degrees is {} Celsius degrees",
                        temperature,
                        fahrenheit_to_celsius(temperature)
                    );
                    break;
                }
                "Fahrenheit" => {
                    println!(
                        "{} Celsius degrees is {} Fahrenheit degrees",
                        temperature,
                        celsius_to_fahnreheit(temperature)
                    );
                    break;
                }
                _ => {
                    println!("Something went wrong..");
                    continue;
                }
            }
        }
        break;
    }
}
