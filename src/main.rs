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
	let img = if config.resize {
		resize(open_img(filename()))
	} else {
		open_img (filename()).into_rgba8()
	};
	
	let config = Config::default();
	imgops::print_img(img);
}
