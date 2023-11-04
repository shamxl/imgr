mod fileops;
mod imgops;
mod config;
mod output;
use fileops::filename;
use fileops::open_img;
use imgops::resize;
use imgops::get_luminance;
use config::{Config, Styles};
use image::GenericImageView;

fn main() {
	let config = Config::default();
	let style = config.style;
	let img = resize(open_img (filename()));
	let (width, height) = img.dimensions();
	let scale = config.scale;
	
	imgops::print_img(img)
}
