use std::process;

pub fn help() {
    let help_text =
    "
    booru_cli is a CLI script that allows you to download images in batches from booru sites.\n
    \n
    Usage: cargo run -- [BOORU] <amount> <tags> [OPTIONS]\n
    \n
    [BOORU]\n
    gelbooru, -g        Get images from gelbooru.com\n
    safebooru, -s       Get images from safebooru.org\n
    [OPTIONS]\n
    Check respective booru help command for detailed list of options.\n
    \n
    Help command:\n
    \n
    help, -h            Will show you this print\n
    safebooru help/-h   Will show you the options for safebooru\n
    gelbooru help/-h    Will show you the options for gelbooru\n
    ";

    println!("{help_text}");
    process::exit(0x0100);
}

pub fn gelbooru_options() {
    let options_text = 
    "
    [OPTIONS]\n
    nsfw, -n            This will return only nsfw images\n
    sfw, -s             This will return only sfw images\n
    If you want to download any type of image don't add any of the above options\n
    ";

    println!("{options_text}");
    process::exit(0x0100);
}

pub fn safebooru_options() {
    let options_text = 
    "
    [OPTIONS]\n
    questionable, -q    This will return only questionable images\n
    sfw, -s             This will return only sfw images\n
    If you want to download any type of image don't add any of the above options\n
    ";

    println!("{options_text}");
    process::exit(0x0100);
}