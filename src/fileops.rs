use image::{DynamicImage};
use std::path::Path;
use crate::config::Config;
use clap::Parser;

pub fn open_img () -> DynamicImage {
	let config = Config::parse();
	let img = image::open(Path::new(&config.filename)).unwrap();
	img
}
