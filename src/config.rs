use clap::{Parser, ValueEnum};


#[derive(Parser, Debug)]
#[command(author, version, about = "image to ascii converter", long_about = None)]
pub struct Config {
    #[arg(value_enum, short, long, default_value = "ascii")]
    pub style: Styles,

    /// path of the image
    pub filename: String,

    /// to set the scale of image
    #[arg(short = 'S', long, default_value = "5")]
    pub scale: u32,

    /// to set the output color
    #[arg(short, long, default_value = "false")]
    pub colored: bool,

    /// to set wheather the image to be resized or not
    #[arg(short, long, default_value = "false")]
    pub resize: bool,

    /// to set the char of block style
    #[arg(short, long, default_value = "█")]
    pub block: char,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Styles {
    /// print as ascii chars
    Ascii,
    /// print as block
    Block,
    /// print as braille - ! Experimental (use with color on)
    Braille
}
