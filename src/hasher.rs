extern crate image;
extern crate img_hash;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use img_hash::HasherConfig;
use rayon::prelude::*;

use std::error::Error;

use crate::args::Args;

use crate::Image;

// Generates the phash for each image
fn get_image_phash(file: &String) -> Result<String, Box<dyn Error>> {
    let img_bytes = reqwest::blocking::get(file)?.bytes()?;
    let hasher = HasherConfig::new().to_hasher();
    let image = image::load_from_memory(&img_bytes)?;

    Ok(hasher.hash_image(&image).to_base64())
}

// Read each line from the file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// request the phash for each image and write them to a new Image vector
fn get_images_to_phash(filename: &String) -> Vec<Image> {
    let mut images: Vec<Image> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(url) = line {
                images.push(Image {
                    url,
                    phash: String::from(""),
                })
            }
        }
    }

    images
}

// Takes the filename argument, requests each image URL, generates the
// perceptual hash for them, and returns a vector of the Image structs.
pub fn hash_images(args: Args) -> Vec<Image> {
    let mut selected_image_map = get_images_to_phash(&args.input);

    selected_image_map.par_iter_mut().for_each(|image| {
        let phash = match get_image_phash(&image.url) {
            Ok(hash) => hash,
            _ => String::from(""),
        };
        image.phash = phash;
        if args.debug {
            println!("Phash for '{}' is {}", &image.url, &image.phash);
        }
    });

    selected_image_map
}
