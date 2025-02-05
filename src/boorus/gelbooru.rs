use std::{process,io::Write};

pub struct GelbooruConfig {
    image_amount: i64,
    tags: String,
    pid: i64,
}

impl GelbooruConfig {
    pub fn build(args: &[String]) -> Result<GelbooruConfig, &'static str> {
        let arg_amount = args.len(); // Save this here to avoid recalculating it everytime
        if arg_amount < 2 {
            return Err("Not enough arguments");
        }
        let image_amount = args[0].clone().parse::<i64>().unwrap();
        let mut tags = args[1].clone();

        let pid;
        if image_amount%100 != 0 {
            pid = image_amount / 100 + 1;
        } else {
            pid = image_amount / 100;
        }

        // handle extra optional args here
        if arg_amount > 2 {
            for i in 2..=arg_amount-1 {
                let current_arg = args[i].as_str();
                match current_arg {
                    "nsfw" | "-n" => tags.push_str(" rating:explicit"),
                    "safe" | "-sfw" => tags.push_str(" rating:general"),
                    "+score" | "+s" => tags.push_str(" sort:score:asc"),
                    "-score" | "-s" => tags.push_str(" sort:score:desc"),
                    "oldest" | "-o" => tags.push_str(" sort:id:asc"),
                    "newest" | "-ns" => tags.push_str(" sort:id:desc"),
                    _ => ()
                }
            }
        }

        Ok(GelbooruConfig{
                image_amount,
                tags,
                pid,
            }  
        )   
    }
}

pub fn run_gelbooru(config: GelbooruConfig) {
    let _ = check_file_path().unwrap_or_else(|err| {
        eprintln!("Problem with download directory: {err}");
        process::exit(1);
    });

    let tags = config.tags;
    let mut images_left = config.image_amount;

    for page in 0..config.pid {
        // Format the get request using given parameters
        let get_request = format!("https://gelbooru.com/index.php?page=dapi&json=1&s=post&q=index&limit={}&tags={}&pid={}", images_left, tags, page);
        // Get image urls
        let images = get_images(get_request);
        let length = images.len() as i64;
        if length < images_left { images_left = length};
        images_left = download_images(images, images_left);
    }
    println!("\r\nFinished! You can find the images in images/gelbooru");
}

#[tokio::main]
async fn get_images(get_request: String) -> Vec<GelbooruPost>{
    let response = reqwest::get(get_request)
                .await
                .unwrap()
                .json::<GelbooruAPI>()
                .await
                .unwrap();
    
    response.post
}

fn download_images(posts: Vec<GelbooruPost>, mut images_left: i64) -> i64 {
    for post in posts {
        let image = post;

        // Little print so you can see progress in the CLI
        print!("\rImages left to download: {images_left}    ");
        let _ = std::io::stdout().flush();
        
        // Get file extension
        let (_, file_extension) = image.file_url.rsplit_once(".").unwrap();
        
        // Format the filename
        let image_id = image.id.to_string();
        let file_name = format!("images/gelbooru/{image_id}.{file_extension}");

        // Create the file to store the image
        let mut file = std::fs::File::create(file_name).unwrap();
        reqwest::blocking::get(image.file_url)
            .unwrap()
            .copy_to(&mut file)
            .unwrap();

        images_left -= 1;
    }
    images_left
}

// TODO: skip amount of images

fn check_file_path() -> std::io::Result<()>{
    match std::fs::exists("images/gelbooru/") {
        Ok(true) => (),
        Ok(false) => {
            println!("Making new folder to save images to (images/gelbooru)");
            std::fs::create_dir("images/gelbooru")?;
        }
        Err(err) => { return Err(err);}
    }
    Ok(())
}

// API structs so the responses are turned into structs which are nicer to deal with
#[derive(serde::Deserialize, Debug)]
struct GelbooruAPI {
	post: Vec<GelbooruPost>,
}

#[derive(serde::Deserialize, Debug)]
struct GelbooruPost {
    file_url: String,
    id: i64
}