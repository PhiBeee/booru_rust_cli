use std::process;

pub struct GelbooruConfig {
    image_amount: i64,
    tags: String
}

impl GelbooruConfig {
    pub fn build(args: &[String]) -> Result<GelbooruConfig, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let image_amount = args[1].clone().parse::<i64>().unwrap();
        let tags = args[2].clone();

        Ok(GelbooruConfig{
                image_amount,
                tags,
            }  
        )   
    }
}

pub fn run(config: GelbooruConfig) {
    // Format the get request using given parameters
    let get_request = format!("https://gelbooru.com/index.php?page=dapi&json=1&s=post&q=index&limit={}&tags={}", config.image_amount, config.tags);

    // Get image urls
    let images = get_images(get_request);
    download_images(images);
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

fn download_images(posts: Vec<GelbooruPost>) {
    let _ = check_file_path().unwrap_or_else(|err| {
        eprintln!("Problem with download directory: {err}");
        process::exit(1);
    });

    for post in posts {
        let image = post;
        
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
    }
}

fn check_file_path() -> std::io::Result<()>{
    match std::fs::exists("images/gelbooru/") {
        Ok(true) => println!("Download folder exists already (images/gelbooru)"),
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