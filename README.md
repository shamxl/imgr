# imgr

An image to ascii converter written in rust 🦀

# Usage

```
imgr [options] <path to image>
```

# Options


## Color

To enable colored output

`-c` or `--colored`

## Styles

To set the style of image

`-s <style>` or `--style <style>`

Available options:
- ascii (default)
- block
- braille (experimental)

## Resize

To enable image resizing

`-r` or `--resize`

**also set the resizing scale using:**

`-S` or `--scale`

default set to: `5`


# Installation

### Cargo

```
cargo install imgr
```

### From source

```
git clone https://github.com/shamxl/imgr.git
cd imgr
cargo build --release --path .
```

<br>

> [!NOTE]
> As I continue my journey with Rust, please pardon any imperfections in this program – they're all part of the learning process.
