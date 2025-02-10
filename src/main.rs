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
    else {
        match args[1].as_str() {
            "help" | "-h" => help(),
            "safebooru" | "-s" => {
                check_args_for_booru(0, &args);
            },
            "gelbooru" | "-g" => {
                check_args_for_booru(1, &args);
            },
            "e621" | "-e" => {
                check_args_for_booru(2, &args);
            },
            "konachan" | "-k" => {
                check_args_for_booru(3, &args);
            },
            "danbooru" | "-d" => {
                check_args_for_booru(4, &args);
            }
            _ => {
                println!("Please specify which booru to use ");
                process::exit(1);
            }
        }    
    }
}

fn check_args_for_booru(booru: i8, args: &[String]) {
    // Avoid IOB
    if args.len() > 2 {
        if args[2].as_str() == "help" || args[2].as_str() == "-h" {
            match booru {
                0 => safebooru_options(),
                1 => gelbooru_options(),
                2 => e621_options(),
                3 => konachan_options(),
                4 => danbooru_options(),
                _ => (),
            }
        }
        else {
            match booru {
                0 => {
                    let config = BooruConfig::build(&args[2..], 1000, 0).unwrap_or_else(|err|{
                        eprintln!("Problem parsing arguments: {err}");
                        process::exit(1);
                    });
                    run_safebooru(config);
                },
                1 => {
                    let config = BooruConfig::build(&args[2..], 100, 1).unwrap_or_else(|err|{
                        eprintln!("Problem parsing arguments: {err}");
                        process::exit(1);
                    });
                    run_gelbooru(config);
                },
                2 => {
                    let config = BooruConfig::build(&args[2..], 320, 2).unwrap_or_else(|err|{
                        eprintln!("Problem parsing arguments: {err}");
                        process::exit(1);
                    });
                    run_e621(config);
                },
                3 => {
                    let config = BooruConfig::build(&args[2..], 1000, 3).unwrap_or_else(|err|{
                        eprintln!("Problem parsing arguments: {err}");
                        process::exit(1);
                    });
                    run_konachan(config);
                },
                4 => {
                    let config = BooruConfig::build(&args[2..], 200, 4).unwrap_or_else(|err|{
                        eprintln!("Problem parsing arguments: {err}");
                        process::exit(1);
                    });
                    run_danbooru(config);
                }
                _ => (), // This function should never land here 
            }
        }
    }
    else {
        eprintln!("Please specify amount and tags or help command.");
        process::exit(1);
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




