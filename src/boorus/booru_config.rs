use std::process;
// use reqwest::header::*;

pub struct BooruConfig {
    pub image_amount: i64,
    pub tags: String,
    pub pid: i64,
    pub images_to_skip: i64,
}

impl BooruConfig {
    pub fn build(args: &[String], req_cap: i64, booru: i8) -> Result<BooruConfig, &'static str> {
        let arg_amount = args.len(); // Save this here to avoid recalculating it everytime
        if arg_amount < 2 {
            return Err("Not enough arguments");
        }
        let image_amount = args[0].parse::<i64>().unwrap_or_else(|_|{
            eprintln!("Your first parameter is not a number!");
            process::exit(1);
        });
        let mut tags = args[1].clone();

        let mut images_to_skip= 0;
        // handle extra optional args here
        if arg_amount > 2 {
            match booru {
                0 => {
                    tags = safebooru_extra_args(args, &mut images_to_skip, tags, arg_amount);
                },
                1 => {
                    tags = gelbooru_extra_args(args, &mut images_to_skip, tags, arg_amount);
                },
                2 => {
                    tags = e621_extra_args(args, &mut images_to_skip, tags, arg_amount);
                },
                3 => {
                    tags = konachan_extra_args(args, &mut images_to_skip, tags, arg_amount);
                }
                _ => {
                    eprintln!("Soemthing went horribly wrong we should not be here");
                    process::exit(1);
                }
            }
        }

        // Figure out how many pages we need
        let mut pid;
        if image_amount%req_cap != 0 {
            pid = image_amount / req_cap + 1;
        } else {
            pid = image_amount / req_cap;
        }

        // Skipping pages requires to go further down the pages
        if images_to_skip%req_cap != 0 {
            if images_to_skip > req_cap {
                pid += images_to_skip / req_cap + 1;
            }
        } else {
            pid += images_to_skip / req_cap;
        }

        // Handle download path
        let download_path: String;
        match booru {
            0 => download_path = String::from("images/safebooru"),
            1 => download_path = String::from("images/gelbooru"),
            2 => download_path = String::from("images/e621"),
            3 => download_path = String::from("images/konachan"),
            _ => {
                eprintln!("Soemthing went horribly wrong we should not be here");
                process::exit(1);
            }
        }

        check_booru_filepath(download_path).unwrap_or_else(|err| {
            eprintln!("Problem with download directory: {err}");
            process::exit(1);
        });

        Ok(BooruConfig{
                image_amount,
                tags,
                pid,
                images_to_skip,
            }  
        )   
    }
}

fn gelbooru_extra_args(args: &[String], images_to_skip: &mut i64, mut tags: String, arg_amount: usize) -> String {
    for i in 2..=arg_amount-1 {
        let current_arg = args[i].as_str();
        match current_arg {
            "nsfw" | "-n"         => tags.push_str(" rating:explicit"),
            "safe" | "-sfw"       => tags.push_str(" rating:general"),
            "+score" | "+s"       => tags.push_str(" sort:score:asc"),
            "-score" | "-s"       => tags.push_str(" sort:score:desc"),
            "oldest" | "-o"       => tags.push_str(" sort:id:asc"),
            "newest" | "-ns"      => tags.push_str(" sort:id:desc"),
            "skip"                => {
                // Make sure there is at least one more arg in the array
                if arg_amount >= i+2 { 
                    *images_to_skip = args[i+1].clone().parse::<i64>().unwrap_or_else(|err| {
                        eprintln!("Please specify an amount of images to be skipped: {err}");
                        process::exit(1);
                    })
                }
                // Let the user know 
                else { println!("No amount was given after the skip option, no images will be skipped") };
            }
            _ => ()
        }
    }
    tags
}

fn safebooru_extra_args(args: &[String], images_to_skip: &mut i64, mut tags: String, arg_amount: usize) -> String {
    for i in 2..=arg_amount-1 {
        let current_arg = args[i].as_str();
        match current_arg {
            "nsfw" | "-n"         => tags.push_str(" rating:explicit"),
            "safe" | "-sfw"       => tags.push_str(" rating:general"),
            "+score" | "+s"       => tags.push_str(" sort:score:asc"),
            "-score" | "-s"       => tags.push_str(" sort:score:desc"),
            "oldest" | "-o"       => tags.push_str(" sort:id:asc"),
            "newest" | "-ns"      => tags.push_str(" sort:id:desc"),
            "skip"                => {
                // Make sure there is at least one more arg in the array
                if arg_amount >= i+2 { 
                    *images_to_skip = args[i+1].clone().parse::<i64>().unwrap_or_else(|err| {
                        eprintln!("Please specify an amount of images to be skipped: {err}");
                        process::exit(1);
                    })
                }
                // Let the user know 
                else { println!("No amount was given after the skip option, no images will be skipped") };
            }
            _ => ()
        }
    }
    tags
}

fn e621_extra_args(args: &[String], images_to_skip: &mut i64, mut tags: String, arg_amount: usize) -> String {
    for i in 2..=arg_amount-1 {
        let current_arg = args[i].as_str();
        match current_arg {
            "safe" | "-sfw"       => tags.push_str(" rating:general"),
            "questionable" | "-q" => tags.push_str(" rating:questionable"),
            "explicit" | "-e"     => tags.push_str(" rating:explicit"),
            "oldest" | "-o"       => tags.push_str(" order:id"),
            "favorites" | "-f"    => tags.push_str(" order:favcount"),
            "score" | "-s"        => tags.push_str(" order:score"),
            "jpg"                 => tags.push_str(" type:jpg"),
            "png"                 => tags.push_str(" type:png"),  
            "webm"                => tags.push_str(" type:webm"),
            "gif"                 => tags.push_str(" type:gif"),   
            "skip"                => {
                // Make sure there is at least one more arg in the array
                if arg_amount >= i+2 { 
                    *images_to_skip = args[i+1].clone().parse::<i64>().unwrap_or_else(|err| {
                        eprintln!("Please specify an amount of images to be skipped: {err}");
                        process::exit(1);
                    })
                }
                // Let the user know 
                else { println!("No amount was given after the skip option, no images will be skipped") };
            }
            _ => ()
        }
    }
    tags
}

fn konachan_extra_args(args: &[String], images_to_skip: &mut i64, mut tags: String, arg_amount: usize) -> String {
    for i in 2..=arg_amount-1 {
        let current_arg = args[i].as_str();
        match current_arg {
            "safe" | "-sfw"       => tags.push_str(" rating:safe"),
            "questionable" | "-q" => tags.push_str(" rating:questionable"),
            "+score" | "+s"       => tags.push_str(" order:score_asc"),
            "-score" | "-s"       => tags.push_str(" order:score"),
            "oldest" | "-o"       => tags.push_str(" order:id"),
            "newest" | "-ns"      => tags.push_str(" order:id_desc"),
            "landscape" | "-l"    => tags.push_str(" order:landscape"),
            "portrait" | "-p"     => tags.push_str(" order:portrait"),
            "skip"                => {
                // Make sure there is at least one more arg in the array
                if arg_amount >= i+2 { 
                    *images_to_skip = args[i+1].clone().parse::<i64>().unwrap_or_else(|err| {
                        eprintln!("Please specify an amount of images to be skipped: {err}");
                        process::exit(1);
                    })
                }
                // Let the user know 
                else { println!("No amount was given after the skip option, no images will be skipped") };
            }
            _ => ()
        }
    }
    tags
}

fn check_booru_filepath(download_path: String) -> std::io::Result<()> {
    match std::fs::exists(&download_path) {
        Ok(true) => (),
        Ok(false) => {
            println!("Making new folder to save images to ({download_path})");
            std::fs::create_dir(download_path)?;
        }
        Err(err) => { return Err(err);}
    }
    Ok(())
}