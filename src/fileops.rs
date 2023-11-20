use image::{DynamicImage};
use std::path::Path;
use crate::config::Config;
use clap::Parser;
use std::fs::OpenOptions;
use std::io::Write;

pub fn open_img () -> DynamicImage {
	let config = Config::parse();
	let img = image::open(Path::new(&config.filename)).unwrap();
	img
}


pub fn write_contents<T: AsRef<str>> (content: T, filename: &String) {
	let content = content.as_ref();
	let mut file = OpenOptions::new()
		.write(true)
		.append(true)
		.create(true)
		.open(filename)
		.expect("e: Failed to open file");

	file.write_all(content.as_bytes()).expect("e: Failed to write file");
}
