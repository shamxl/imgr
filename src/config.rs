#[derive(Debug)]
pub enum Styles {
	Ascii,
	Braille,
	Block
}

pub struct Config {
	pub ascii: Vec<char>,
	pub scale: u32,
	pub block: char,
	pub colored: bool,
	pub resize: bool,
	pub style: Styles,
}

impl Default for Config {
	fn default () -> Self {
		Self {
			ascii: vec![' ', '.', ':', 'o', 'O', '#', '@', '@'],
			scale: 3,
			block: std::char::from_u32(9608).unwrap(),
			colored: true,
			resize: true,
			style: Styles::Ascii
		}
	}
}
