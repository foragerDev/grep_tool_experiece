use std::{env, process};
use Grep::{Config, read_file, serach};
use std::fmt;

fn main() {
    let args: Vec<String> = env::args().collect(); //because I am getting iterator so I can call collect which belongs to Vec
    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        print!("Error occured: {}", err);
        process::exit(1);
    });
    let contents = read_file(&config.path);
    let results = serach(&contents, &config.query);
    if results.is_empty(){
        println!("Sorry nothing found");
    }
    for (pos, i) in results.iter().enumerate(){
        println!("{}: {}", pos, i);
    }
}
