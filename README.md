# CLI Booru downloader

There is a lot of tools like this but I haven't found one in Rust, so I just decided to make my own.  
I like to run things from my terminal so that's what this will be.  
# Features
- Specify amount of images to download
- Specify tags, however you want (as long as they exist on the booru)
- Sorting results by rating, upload date and other metrics
- Filter between explicit, questionable and sfw results
# Supported Boorus
- [Safebooru](https://safebooru.org)
- [Gelbooru](https://gelbooru.com)
- [e621](https://e621.net)
# Usage
Download the executable from the releases for your platform [Windows](https://github.com/PhiBeee/booru_rust_cli/releases/download/1.0/booru_cli.exe) or [Linux](https://github.com/PhiBeee/booru_rust_cli/releases/download/1.0/booru_cli)  

Navigate to where the file was downloaded (usually `cd Downloads`) and then
```
booru_cli -h
```
to get a list of commands.  
The basic structure is:
```
booru_cli <booru> <amount> <tags> [OPTIONS]
```
You can find more in-depth commands with the help command listed above.

# Compiling from source

Make sure you have rust installed, install it [here](https://www.rust-lang.org/tools/install) if you don't have it yet.  
  
Clone the repository to some directory (or download the project)  
```
git clone https://github.com/PhiBeee/booru_rust_cli.git
```  
  
Navigate to the directory and run:  
```
cargo build -r
```  
This should create an executable in the target folder.
# To-do's
- Add support for more image websites
