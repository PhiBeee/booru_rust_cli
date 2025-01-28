fn main() {
    let image_limit = 100;
    let tag = "maid";
    let get_request = format!("https://gelbooru.com/index.php?page=dapi&json=1&s=post&q=index&limit={image_limit}&tags={tag}");

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
    for post in posts {
        let image = post;
        
        // Get file extension
        let (_, file_extension) = image.file_url.rsplit_once(".").unwrap();
        
        // Format the filename
        let image_id = image.id.to_string();
        let file_name = format!("images/{image_id}.{file_extension}");

        // Create the file to store the image
        let mut file = std::fs::File::create(file_name).unwrap();
        reqwest::blocking::get(image.file_url)
            .unwrap()
            .copy_to(&mut file)
            .unwrap();
    }
}

#[derive(serde::Deserialize, Debug)]
struct GelbooruAPI {
	post: Vec<GelbooruPost>,
}

#[derive(serde::Deserialize, Debug)]
struct GelbooruPost {
    file_url: String,
    id: i64
}


