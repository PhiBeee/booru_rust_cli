use super::booru_config::BooruConfig;
use std::{process,io::Write};
use reqwest::header::*;

const REQUEST_CAP: i64 = 1000;

pub fn run_konachan(config: BooruConfig) {
    let tags = config.tags;
    let mut images_left = config.image_amount;
    // Rust rounds down no matter what so this is great
    let skipped_pages = config.images_to_skip / REQUEST_CAP;
    // Make sure we only have the amount left that is needed
    let mut images_to_skip = config.images_to_skip - (skipped_pages * REQUEST_CAP);
    // Make sure we get enough images in case the amount requested is lower than the get cap
    if images_left < REQUEST_CAP { images_left += images_to_skip };

    for page in 1..=config.pid {
        // Format the get request using given parameters
        let get_request = format!("https://konachan.net/post.json?limit={}&tags={}&page={}", images_left, tags, page);
        // Get image urls
        let mut images = get_images(get_request);

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
    println!("\r\nFinished! You can find the images in images/konachan");
}

#[tokio::main]
async fn get_images(get_request: String) -> Vec<KonachanPost> {
    let client = reqwest::Client::new();
    let response = client.get(get_request)
                .header(USER_AGENT, "booru_cli/1.2 ")
                .send()
                .await
                .unwrap_or_else(|err|{
                    eprintln!("Error getting a response from the API: {err}");
                    process::exit(1);
                })
                .json::<Vec<KonachanPost>>()
                .await
                .unwrap_or_else(|err|{
                    eprintln!("No posts under the given tags, double check they exist or make the search less specifc: {err}");
                    process::exit(1);
                });
    
    response
}

fn download_images(posts: Vec<KonachanPost>, mut images_left: i64) -> i64{
    for post in posts {
        let image = post;
        // Little print so you can see progress in the CLI
        print!("\rImages left to download: {images_left}    ");
        let _ = std::io::stdout().flush();
        match image.file_url {
            Some(url) => {
                // Get file extension
                let (_, file_extension) = url.rsplit_once(".").unwrap();
                
                // Format the filename
                let image_id = image.id.to_string();
                let file_name = format!("images/konachan/{image_id}.{file_extension}");

                // Create the file to store the image
                let mut file = std::fs::File::create(file_name).unwrap();
                reqwest::blocking::get(url)
                    .unwrap()
                    .copy_to(&mut file)
                    .unwrap();
                },
            None => (),
        }
        images_left -= 1;
    }
    images_left
}

#[derive(serde::Deserialize, Debug)]
struct KonachanPost {
    id: i64,
    file_url: Option<String>
}