use std::{process,io::Write};

pub struct SafebooruConfig {
    image_amount: i64,
    tags: String,
    pid: i64,
}

impl SafebooruConfig {
    pub fn build(args: &[String]) -> Result<SafebooruConfig, &'static str> {
        let arg_amount = args.len(); // Save this here to avoid recalculating it everytime
        if arg_amount < 2 {
            return Err("Not enough arguments");
        }
        let image_amount = args[0].clone().parse::<i64>().unwrap();
        let mut tags = args[1].clone();

        let pid;
        if image_amount%1000 != 0 {
            pid = image_amount / 1000 + 1;
        } else {
            pid = image_amount / 1000;
        }

        // handle extra optional args here
        if arg_amount > 2 {
            for i in 2..=arg_amount-1 {
                let current_arg = args[i].as_str();
                match current_arg {
                    "safe" | "-sfw" => tags.push_str(" rating:general"),
                    "questionable" | "-q" => tags.push_str(" rating:questionable"),
                    _ => ()
                }
            }
        }

        Ok(SafebooruConfig{
                image_amount,
                tags,
                pid,
            }  
        )   
    }
}

pub fn run_safebooru(config: SafebooruConfig) {
    let _ = check_file_path().unwrap_or_else(|err| {
        eprintln!("Problem with download directory: {err}");
        process::exit(1);
    });

    let tags = config.tags;
    let mut images_left = config.image_amount;

    for page in 0..config.pid {
        // Little print so you can see progress in the CLI (Doesn't really do much for this one because we get images in batches of 1000)
        print!("\rImages left to download: {images_left}");
        let _ = std::io::stdout().flush();

        // Format the get request using given parameters
        let get_request = format!("https://safebooru.org/index.php?page=dapi&json=1&s=post&q=index&limit={}&tags={}&pid={}", images_left, tags, page);
        // Get image urls
        let images = get_images(get_request);
        download_images(images);
        images_left -= 1000;
    }
    println!("\r\nFinished! You can find the images in images/safebooru");
}

#[tokio::main]
async fn get_images(get_request: String) -> Vec<SafebooruPost>{
    let response = reqwest::get(get_request)
                .await
                .unwrap()
                .json::<Vec<SafebooruPost>>()
                .await
                .unwrap();
    
    response
}

fn download_images(posts: Vec<SafebooruPost>) {
    for post in posts {
        let image = post;
        
        // Get file extension
        let (_, file_extension) = image.file_url.rsplit_once(".").unwrap();
        
        // Format the filename
        let image_id = image.id.to_string();
        let file_name = format!("images/safebooru/{image_id}.{file_extension}");

        // Create the file to store the image
        let mut file = std::fs::File::create(file_name).unwrap();
        reqwest::blocking::get(image.file_url)
            .unwrap()
            .copy_to(&mut file)
            .unwrap();
    }
}

fn check_file_path() -> std::io::Result<()>{
    match std::fs::exists("images/safebooru/") {
        Ok(true) => (),
        Ok(false) => {
            println!("Making new folder to save images to (images/safebooru)");
            std::fs::create_dir("images/safebooru")?;
        }
        Err(err) => { return Err(err);}
    }
    Ok(())
}


#[derive(serde::Deserialize, Debug)]
struct SafebooruPost {
    id: i64,
	file_url: String,
}
