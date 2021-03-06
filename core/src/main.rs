use clients::rest_client;
use json_color::{Color, Colorizer};
use std::{env, process::exit};

pub mod clients;
pub mod file_writer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.get(1).is_none() {
        println!("the -command arg is missing!\nuse -h for help");
        exit(-1)
    }

    let command = args.get(1).unwrap().as_str();

    if command.eq("-r") || command.eq("-R")  || command.eq("-rw") || command.eq("-RW") {

        //json color config
        let colorizer = Colorizer::new()
            .null(Color::Purple)
            .boolean(Color::Magenta)
            .number(Color::Red)
            .string(Color::Green)
            .key(Color::Blue)
            .build();

        let second_arg = args.get(2);
        let third_arg = args.get(3);

        if second_arg.is_some() {
            //REST call
            match rest_client::rest_call(second_arg.unwrap()) {
                Ok(value) => {
                    let colorized_value = colorizer.colorize_json_str(&value);
                    println!("{}", format!("Values: {}", colorized_value.unwrap()));

                    if third_arg.is_some() {
                        if command.eq("-rw") || command.eq("-RW") {
                            let _ = file_writer::write(&value, &third_arg.unwrap());
                        }
                    } else {
                        println!("unwnown arg!\n use -h for help")
                    }
                }
                Err(e) => {
                    println!("{}", format!("Error: {}", e))
                }
            }
        } else {
            println!("the url arg is missing!\n use -h for help")
        }
    } else if command.eq("-h") || command.eq("-H") {
        println!("the following commands are avaliable:\n
         -r --> REST api call\n
         -h --> show help\n
         -rw --> write result to file given a file path\n
         example for -r: ./ccurl -r https://jsonplaceholder.typicode.com/todos/1")
    } else {
        println!("error, unknown parameters!\nuse -h for help")
    }
}
