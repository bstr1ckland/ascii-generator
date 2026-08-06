use image::{ImageReader, ImageError};
use indoc::indoc;
use std::env;
// use std::io::Cursor;
use std::process;

fn main() -> Result<(), ImageError> {
    let args: Vec<String> = env::args().collect();
    let help: &str = indoc! {"
        Usage: $ cargo run -- [help] [path]
        Options:
            help , shows this help text
            path , path to image to convert to ascii 
    "};
   
    // First arg is target/debug/<project-name>, in this case it's target/debug/assci
    if args.len() != 2 {
        eprintln!("Error: Invalid amount of arguments. Run `cargo run help` for info");
        process::exit(1);
    }

    if &args[1] == "help" {
        println!("{help}");
    }

    let img_path: &String = &args[1];
    let image = ImageReader::open(img_path)?.decode()?;

    Ok(())
}
