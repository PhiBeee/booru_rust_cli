use std::env;
use std::process;

mod boorus;
use boorus::*;

mod help;
use help::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "-h" || args[1] == "help"{
        help();
        process::exit(1);
    }

    let config = GelbooruConfig::build(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if check_main_folder_existance().is_err() {
        eprintln!("Issue creating main folder for images, check permissions");
        process::exit(1);
    }

    run(config);   
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




