use image::imageops;
use image::{ImageReader, ImageError};
use indoc::indoc;
use ndarray::Array2;
use std::env;
use std::process;

fn main() -> Result<(), ImageError> {
    let args: Vec<String> = env::args().collect();
    let help = indoc! {"
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

    let img_path = &args[1];
    // Store an convert image to grayscale
    let image = ImageReader::open(img_path)?.decode()?.grayscale();
    let mut grayscale_image = image::imageops::colorops::grayscale(&image);

    // Scale height down, since one pixel = ~1/2 ascii character
    let width = 50 as usize;
    let height = 25 as usize;

    grayscale_image = imageops::thumbnail(&grayscale_image, width as u32, height as u32);
    
    let mut ascii_image = Array2::<char>::from_elem((height, width), ' ');

    // pixel is a Luma<u8>
    for (x, y, image::Luma([gray_value])) in grayscale_image.enumerate_pixels() {
        let ascii_char = match gray_value {
            0..=24 => ' ',
            25..=49 => '.',
            50..=74 => ':',
            75..=99 => '-',
            100..=124 => '=',
            125..=149 => '+',
            150..=174 => '*',
            175..=199 => '#',
            200..=224 => '%',
            _ => '@',
        };
        ascii_image[[y as usize, x as usize]] = ascii_char;
    }

    for row in ascii_image.rows() {
        let line: String = row.iter().collect();
        println!("{}", line);
    }

    Ok(())
}
