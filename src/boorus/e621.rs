use std::{process,io::Write};
use reqwest::header::*;

pub struct E621Config {
    image_amount: i64,
    tags: String,
    pid: i64,
}

impl E621Config {
    pub fn build(args: &[String]) -> Result<E621Config, &'static str> {
        let arg_amount = args.len(); // Save this here to avoid recalculating it everytime
        if arg_amount < 2 {
            return Err("Not enough arguments");
        }
        let image_amount = args[0].clone().parse::<i64>().unwrap();
        let mut tags = args[1].clone();

        let pid;
        if image_amount%200 != 0 {
            pid = image_amount / 200 + 1;
        } else {
            pid = image_amount / 200;
        }

        // handle extra optional args here
        if arg_amount > 2 {
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
                    _ => ()
                }
            }
        }

        Ok(E621Config{
                image_amount,
                tags,
                pid,
            }  
        )   
    }
}

pub fn run_e621(config: E621Config) {
    let tags = config.tags;

    for page in 1..=config.pid {
        // Little print so you can see progress in the CLI
        let page_print = page*2;
        print!("\rCurrently downloading up to image: {page_print}00");
        let _ = std::io::stdout().flush();

        println!("{tags}");

        // Format the get request using given parameters
        let get_request = format!("https://e621.net/posts.json?limit={}&tags={}&page={}", config.image_amount, tags, page);
        // test_request(get_request);
        // Get image urls
        let images = get_images(get_request);
        download_images(images);
    }
    println!("\r\nFinished! You can find the images in images/e621");
}

#[tokio::main]
async fn get_images(get_request: String) -> E621Posts {
    let client = reqwest::Client::new();
    let response = client.get(get_request)
                .header(USER_AGENT, "booru_cli/0.1 (by AnotherDogGirl on e621)")
                .send()
                .await
                .unwrap()
                .json::<E621Posts>()
                .await
                .unwrap();
    
    response
}

fn download_images(posts: E621Posts ) {
    let _ = check_file_path().unwrap_or_else(|err| {
        eprintln!("Problem with download directory: {err}");
        process::exit(1);
    });

    for post in posts.posts {
        let image = post;

        match image.file.url {
            Some(url) => {
                // Get file extension
                let (_, file_extension) = url.rsplit_once(".").unwrap();
                
                // Format the filename
                let image_id = image.id.to_string();
                let file_name = format!("images/e621/{image_id}.{file_extension}");

                // Create the file to store the image
                let mut file = std::fs::File::create(file_name).unwrap();
                reqwest::blocking::get(url)
                    .unwrap()
                    .copy_to(&mut file)
                    .unwrap();
                },
            None => (),
        }
    }
}

fn check_file_path() -> std::io::Result<()>{
    match std::fs::exists("images/e621/") {
        Ok(true) => (),
        Ok(false) => {
            println!("Making new folder to save images to (images/e621)");
            std::fs::create_dir("images/e621")?;
        }
        Err(err) => { return Err(err);}
    }
    Ok(())
}

#[derive(serde::Deserialize, Debug)]
struct E621Posts {
    posts: Vec<E621Post>
}

#[derive(serde::Deserialize, Debug)]
struct E621Post {
    id: i64,
	file: E621File,
}

#[derive(serde::Deserialize, Debug)]
struct E621File {
    url: Option<String>,
}
