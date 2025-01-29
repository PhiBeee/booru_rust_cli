use std::env;
use std::process;

mod boorus;
use boorus::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = GelbooruConfig::build(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    run(config);   
}




