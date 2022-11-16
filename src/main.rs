use clap::Parser;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Image {
    pub url: String,
    pub phash: String,
}

pub mod args;
pub mod hasher;

use crate::{args::Args, hasher::hash_images};

fn main() {
    let args = Args::parse();
    let hashed_images = hash_images(args);

    for image in hashed_images {
        println!("{},{}", image.phash, image.url);
    }
}
