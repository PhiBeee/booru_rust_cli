pub fn help() {
    let help_text =
    "
    booru_cli is a CLI script that allows you to download images in batches from booru sites.\n
    \n
    Usage: cargo run -- <amount> <tags> [OPTIONS]\n
    \n
    [OPTIONS]\n
    nsfw, -n            This will return only nsfw images\n
    sfw, -s             This will return only sfw images\n
    If you want to download any type of image don't add any of the above options\n
    \n
    Help command:\n
    \n
    help, -h            Will show you this print\n
    ";

    println!("{help_text}");
}