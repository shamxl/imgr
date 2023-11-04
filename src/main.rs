mod fileops;
mod imgops;
mod config;
mod output;
use fileops::open_img;
use imgops::resize;
use config::{Config};
use clap::Parser;

fn main() {
	let config = Config::parse();
	let img = if config.resize {
		resize(open_img())
	} else {
		open_img ().into_rgba8()
	};

	imgops::print_img(img);


	
}
