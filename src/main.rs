use std::env;
use std::fs;
use std::process;
pub mod Config;
use Config::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::structs::Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    run(config);

}

