pub fn help() {
    let help_text =
    "
    booru_cli is a CLI script that allows you to download images in batches from booru sites.\n
    \n
    Usage: booru_cli <booru> <amount> <tags> [OPTIONS]\n
    \n
    <BOORU>\n
    gelbooru, -g        Get images from gelbooru.com\n
    safebooru, -s       Get images from safebooru.org\n
    e621, -e            Get images from e621.net\n
    konachan, -k        Get images from konachan.net\n
    danbooru, -d        Get imagess from danbooru.donmai.us\n
    [OPTIONS]\n
    skip <amount>       Skip the first <amount> images\n
    Check the respective booru help command for detailed list of options.\n
    \n
    Help commands:\n
    \n
    help, -h            Will show you this print\n
    safebooru help/-h   Will show you the options for safebooru\n
    gelbooru help/-h    Will show you the options for gelbooru\n
    e621 help/-h        Will show you the options for e621\n
    konachan help/-h    Will show you the options for konachan\n
    danbooru help/-h    Will show you the options for danbooru\n
    ";

    println!("{help_text}");
}

pub fn gelbooru_options() {
    let options_text = 
    "
    [OPTIONS]\n
    nsfw, -n            This will return only nsfw images\n
    safe, -sfw          This will return only sfw images\n
    If you want to download any type of image don't add any of the above options\n
    Filtering:\n
    +score, +s          This will sort by ascending score, giving you posts with the lowest score first\n
    -score, -s          This will sort by descending score, giving you posts with the highest score first\n
    oldest, -o          This will return the earliest posts first\n
    newest, -ns         This will return the newest posts first\n
    ";

    println!("{options_text}");
}

pub fn safebooru_options() {
    let options_text = 
    "
    [OPTIONS]\n
    questionable, -q    This will return only questionable images\n
    safe, -sfw          This will return only sfw images\n
    If you want to download any type of image don't add any of the above options\n
    Filtering:\n
    +score, +s          This will sort by ascending score, giving you posts with the lowest score first\n
    -score, -s          This will sort by descending score, giving you posts with the highest score first\n
    oldest, -o          This will return the earliest posts first\n
    newest, -ns         This will return the newest posts first\n
    ";

    println!("{options_text}");
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
}

pub fn konachan_options() {
    let options_text = 
    "
    [OPTIONS]\n
    safe, -sfw          This will return only sfw images\n
    questionable, -q    This will return only sfw images\n
    If you want to download any type of image don't add any of the above options\n
    Filtering:\n
    +score, +s          This will sort by ascending score, giving you posts with the lowest score first\n
    -score, -s          This will sort by descending score, giving you posts with the highest score first\n
    oldest, -o          This will return the earliest posts first\n
    newest, -ns         This will return the newest posts first\n
    landscape, -l       This will return landscape images first\n
    portrait, -p        This will return portrait images first\n
    ";

    println!("{options_text}");
}

pub fn danbooru_options() {
    let options_text = 
    "
    [OPTIONS]\n
    Rating:\n
    safe, -sfw          This will return only sfw images\n
    questionable, -q    This will return only questionable images\n
    sensitive, -sen     This will return only sensitive images\n
    explicit, -e        This will return only explicit images\n
    If you want to download any type of image don't add any of the above options\n
    Filtering:\n
    oldest, -o          This will order results by oldest first\n
    newest, -ns         This will order results by newest first\n
    +score, +s          This will order results by lowest score first\n
    -score, -s          This will order results by highest score first\n
    File type:\n
    You can optionally add one or multiple filetypes to filter by to only download those types.\n
    Supported types: png, jpg, gif, webm\n    
    ";

    println!("{options_text}");
}