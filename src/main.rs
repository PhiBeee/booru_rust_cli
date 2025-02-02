use std::env;
use std::process;

mod boorus;
use boorus::*;

mod help;
use help::*;

fn main() {
    if check_main_folder_existance().is_err() {
        eprintln!("Issue creating main folder for images, check permissions");
        process::exit(1);
    }

    let args: Vec<String> = env::args().collect();
    // print out help message if no commands were provided
    if args.len() < 2 { help(); }

    match args[1].as_str() {
        "help" | "-h" => help(),
        "gelbooru" | "-g" => {
            // Make sure user gave enough parameters to avoid IOB
            if args.len() > 2 {
                // Check if user wanted to get the help message instead
                match args[2].as_str() {
                    "help" | "-h" => gelbooru_options(),
                    _ => (),
                }
                let config = GelbooruConfig::build(&args[2..]).unwrap_or_else(|err|{
                    eprintln!("Problem parsing arguments: {err}");
                    process::exit(1);
                });
            
                run_gelbooru(config);  
            }
            else {
                eprintln!("Please specify amount and tags or help command.");
                process::exit(1);
            }
        },
        "safebooru" | "-s" => {
            // Make sure user gave enough parameters to avoid IOB
            if args.len() > 2 {
                // Check if user wanted to get the help message instead
                match args[2].as_str() {
                    "help" | "-h" => safebooru_options(),
                    _ => (),
                }
                let config = SafebooruConfig::build(&args[2..]).unwrap_or_else(|err|{
                    eprintln!("Problem parsing arguments: {err}");
                    process::exit(1);
                });
            
                run_safebooru(config); 
            }
            else {
                eprintln!("Please specify amount and tags or help command.");
                process::exit(1);
            }
        },
        "e621" | "-e" => {
            if args.len() > 2 {
                match args[2].as_str() {
                    "help" | "-h" => e621_options(),
                    _ => (),
                }
                let config = E621Config::build(&args[2..]).unwrap_or_else(|err|{
                    eprintln!("Problem parsing arguments: {err}");
                    process::exit(1);
                });

                run_e621(config);
            }
            else {
                eprintln!("Please specify amount and tags or help command.");
                process::exit(1);
            }
        },
        _ => {
            println!("Please specify which booru to use ");
            process::exit(1);
        }
    }    
}

// Checks if the main folder exists and if it does't makes it
fn check_main_folder_existance() -> std::io::Result<()> {
    match std::fs::exists("images/") {
        Ok(true) => (),
        Ok(false) => {
            println!("Making main images folder (/images)");
            std::fs::create_dir("images")?;
        }
        Err(err) => { return Err(err);}
    }

    Ok(())
}




