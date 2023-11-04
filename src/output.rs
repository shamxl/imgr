use crate::config::Config;
use crate::imgops::colored;
use image::Rgba;

pub fn ascii (luminance: u32, pixel: &Rgba<u8>) {
	let config = Config::default();
	let table = config.ascii;
	let lumi = luminance / 32;
	let ascii_char = table[lumi as usize];
	if config.colored {
		print! ("{}", colored(pixel[0], pixel[1], pixel[2], ascii_char));
	} else {
		print! ("{}", ascii_char);
	}
}

pub fn braille (luminance: u32, pixel: &Rgba<u8>) {
	let config = Config::default();
	let braille_char = std::char::from_u32(luminance + 0x2800).unwrap();
	if config.colored {
		print! ("{}", colored(pixel[0], pixel[1], pixel[2], braille_char));
	} else {
		print!("{}", std::char::from_u32(luminance + 0x2800).unwrap());
	}
}

pub fn block (pixel: &Rgba<u8>) {
	let config = Config::default();
	print! ("{}", colored(pixel[0], pixel[1], pixel[2], config.block));
}
