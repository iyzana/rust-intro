use std::io;

fn main() {
    let conversion = loop {
        println!("Convert?");
        println!("1) °C -> °F");
        println!("2) °F -> °C");

        let mut conversion = String::new();
        io::stdin()
            .read_line(&mut conversion)
            .expect("Could not read line!");
        println!("");

        match conversion.trim().parse() {
            Ok(1) => break 1,
            Ok(2) => break 2,
            Ok(_) => continue,
            Err(_) => continue,
        }
    };

    let (input_unit, output_unit) = match conversion {
        1 => ("C", "F"),
        2 => ("F", "C"),
        _ => {
            println!("Unsupported conversion");
            return;
        }
    };

    let input = loop {
        println!("Value in °{}:", input_unit);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read line!");

        match input.trim().parse() {
            Ok(n) => break n,
            Err(_) => {
                println!("");
                continue;
            }
        }
    };

    let output = match conversion {
        1 => convert_to_fahrenheit(input),
        2 => convert_to_celsius(input),
        _ => {
            println!("Unsupported conversion");
            return;
        }
    };

    println!("≅ {} °{}", output, output_unit);
}

fn convert_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn convert_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}
