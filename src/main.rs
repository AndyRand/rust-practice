use rust_practice::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let (query, file_path) = parse_config(&args);
    // let config = parse_config(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Arguments: {:?}", args);
    println!("Query: {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = rust_practice::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
