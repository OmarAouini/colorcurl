use std::env;
use clients::rest_client;
use json_color::{Color, Colorizer};

pub mod clients;

fn main() {

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let command = args.get(1).unwrap().as_str();

    if command.eq("-r") || command.eq("-R") {

    //json color config
    let colorizer = Colorizer::new()
            .null(Color::Purple)
            .boolean(Color::Magenta)
            .number(Color::Red)
            .string(Color::Green)
            .key(Color::Blue)
            .build();

    //REST call
    match rest_client::rest_call(args.get(2).unwrap()) {
        Ok(value) => {
            let colorized_value = colorizer.colorize_json_str(&value);
            println!("{}", format!("Values: {}", colorized_value.unwrap()))
        },
        Err(e) => {println!("{}", format!("Error: {}", e))},
    }
} else {
    println!("error, unknown parameters!")
}
    
}