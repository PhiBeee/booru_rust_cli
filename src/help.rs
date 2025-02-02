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
    e621, -e            Get images from e621.net\n
    [OPTIONS]\n
    Check respective booru help command for detailed list of options.\n
    \n
    Help command:\n
    \n
    help, -h            Will show you this print\n
    safebooru help/-h   Will show you the options for safebooru\n
    gelbooru help/-h    Will show you the options for gelbooru\n
    e621 help/-h        Will show you the options for e621\n
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
    safe, -sfw          This will return only sfw images\n
    If you want to download any type of image don't add any of the above options\n
    ";

    println!("{options_text}");
    process::exit(0x0100);
}

pub fn e621_options() {
    let options_text = 
    "
    [OPTIONS]\n
    Rating:\n
    questionable, -q    This will return only questionable images\n
    safe, -sfw          This will return only sfw images\n
    explicit, -e        This will return only explicit images\n
    If you want to download any type of image don't add any of the above options\n
    Filtering:\n
    oldest, -o          This will order results by oldest first\n
    favorites, -f       This will order results by most favorites first\n
    score, -s           This will order results by highest score first\n
    File type:\n
    You can optionally add one or multiple filetypes to filter by to only download those types.\n
    Supported types: png, jpg, gif, webm\n    
    ";

    println!("{options_text}");
    process::exit(0x0100);
}