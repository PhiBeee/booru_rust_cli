use std::{process,io::Write};

const REQUEST_CAP: i64 = 100; 

pub struct GelbooruConfig {
    image_amount: i64,
    tags: String,
    pid: i64,
    images_to_skip: i64,
}

impl GelbooruConfig {
    pub fn build(args: &[String]) -> Result<GelbooruConfig, &'static str> {
        let arg_amount = args.len(); // Save this here to avoid recalculating it everytime
        if arg_amount < 2 {
            return Err("Not enough arguments");
        }
        let image_amount = args[0].clone().parse::<i64>().unwrap_or_else(|_|{
            eprintln!("Your first parameter is not a number!");
            process::exit(1);
        });
        let mut tags = args[1].clone();

        let mut images_to_skip= 0;
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
                    "skip"                => {
                        // Make sure there is at least one more arg in the array
                        if arg_amount >= i+2 { 
                            images_to_skip = args[i+1].clone().parse::<i64>().unwrap_or_else(|err| {
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
        }

        let mut pid;
        if image_amount%REQUEST_CAP != 0 {
            pid = image_amount / REQUEST_CAP + 1;
        } else {
            pid = image_amount / REQUEST_CAP;
        }

        if images_to_skip%REQUEST_CAP != 0 {
            if images_to_skip > REQUEST_CAP {
                pid += images_to_skip / REQUEST_CAP + 1;
            }
        } else {
            pid += images_to_skip / REQUEST_CAP;
        }

        Ok(GelbooruConfig{
                image_amount,
                tags,
                pid,
                images_to_skip,
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
    // Rust rounds down no matter what so this is great
    let skipped_pages = config.images_to_skip / REQUEST_CAP;
    // Make sure we only have the amount left that is needed
    let mut images_to_skip = config.images_to_skip - (skipped_pages * REQUEST_CAP);
    // Make sure we get enough images in case the amount requested is lower than the get cap
    if images_left < REQUEST_CAP { images_left += images_to_skip };

    for page in 0..config.pid {
        // Format the get request using given parameters
        let get_request = format!("https://gelbooru.com/index.php?page=dapi&json=1&s=post&q=index&limit={}&tags={}&pid={}", images_left, tags, page);
        // Get image urls
        let mut images = get_images(get_request);
        
        // Check if we have reached the end of the posts under the given tags
        let length = images.len() as i64;
        if length < REQUEST_CAP { images_left = length};

        // Remove the amount of images to skip from the results
        if images_to_skip != 0 { 
            images.drain(0..images_to_skip as usize);
            images_left -= images_to_skip;
        };

        images_left = download_images(images, images_left);
        images_to_skip = 0;

        if images_left <= 0 {
            break;
        }
    }
    println!("\r\nFinished! You can find the images in images/gelbooru");
}

#[tokio::main]
async fn get_images(get_request: String) -> Vec<GelbooruPost>{
    let response = reqwest::get(get_request)
                .await
                .unwrap_or_else(|err|{
                    eprintln!("Error getting a response from the API: {err}");
                    process::exit(1);
                })
                .json::<GelbooruAPI>()
                .await
                .unwrap_or_else(|err|{
                    eprintln!("No posts under the given tags, double check they exist or make the search less specifc: {err}");
                    process::exit(1);
                });
    
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