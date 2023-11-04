use crate::config::{Config, Styles};
use crate::output;
use image::{RgbaImage, DynamicImage};
use image::imageops::FilterType::Lanczos3;
use clap::Parser;

pub fn resize (img: DynamicImage) -> RgbaImage {
	let config: Config = Config::parse();
	let scale = config.scale;
	let width = img.width() / scale;
	let height = img.height() / scale;
	let resized_img = img.resize(width, height, Lanczos3);
	if config.colored { 
		resized_img.into_rgba8()
	} else {
		resized_img
			.grayscale()
			.into_rgba8()
	}
}


// luminance formula 
// https://stackoverflow.com/questions/596216/formula-to-determine-perceived-brightness-of-rgb-color
// Y = (0.2126*R + 0.7152*G + 0.0722*B) 
pub fn get_luminance (r: u32, g: u32, b: u32) -> u32 {
	let red = 0.2126 * r as f32;
	let green = 0.7152 * g as f32;
	let blue = 0.0722 * b as f32;

	(red + green + blue) as u32
}


pub fn colored (r: u8, g: u8, b: u8, bchar: char) -> String {
	let prefix = format! ("\x1B[38;2;{r};{g};{b}m{bchar}");
	prefix
}

pub fn print_img (img: RgbaImage) {
	let config = Config::parse();
	let height = img.height();
	let width = img.width();
	let scale = config.scale;
	let style = config.style;

	for y in 0..height {
		for x in 0..width {
			if y % (scale * 2) == 0 && x % scale == 0 {
				let pixel = img.get_pixel(x, y);
				let r: u32 = pixel[0].into();
				let g: u32 = pixel[1].into();
				let b: u32 = pixel[2].into();
				let luminance: u32;
				if pixel[3] == 0 {
					luminance = 0;
				} else {
					luminance = get_luminance(r, g, b);					
				}

				match style {
					Styles::Ascii => output::ascii(luminance, pixel),
					Styles::Block => output::block(pixel),
					Styles::Braille => output::braille(luminance, pixel)
				}
			}
		}
		if y % (scale * 2) == 0 {
			println!();
		}
	}
	
}
