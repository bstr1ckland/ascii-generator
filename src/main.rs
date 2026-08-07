use image::GenericImageView;
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
   
    // First arg is target/debug/<project-name> , with std::env .
    if args.len() != 2 {
        eprintln!("Error: Invalid amount of arguments. Run `cargo run help` for info");
        process::exit(1);
    }

    if &args[1] == "help" {
        println!("{help}");
    }

    let img_path: &String = &args[1];
    let image: image::DynamicImage = ImageReader::open(img_path)?.decode()?.grayscale();
    let brightness_codex: &str = "`.-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@";
    
    let dimensions = image.dimensions();
    // need to downscale image first
    

    for pixel in image.pixels() {
        // 1. find brightness level of the pixel
        // 2. somehow convert that to a character in brightness_codex
        // 3. add the character to a 2d string/character array
        // 4. print the result C:
    }

    Ok(())
}
