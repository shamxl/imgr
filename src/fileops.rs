use std::env;
use image::{DynamicImage};
use std::path::Path;
pub fn filename () -> String {
	if env::args().count() > 1 {
		env::args().nth(1).unwrap()
	} else {
		panic! ("specify an image path")
	}
}

pub fn open_img (path: String) -> DynamicImage {
	let img = image::open(Path::new(&path)).unwrap();
	img
}
