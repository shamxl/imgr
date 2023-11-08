# imgr

A rust-powered image-to-ascii converter 

**Usage:**

```bash
imgr [options] <path to image>
```

**Options:**

**Color:**

- Enable colored output:
  - `-c` or `--colored`

**Styles:**

- Set the style of the image:
  - `-s <style>` or `--style <style>`

Available options:
  - ascii (default)
  - block
  - braille (experimental)

**Resize:**

- Enable image resizing:
  - `-r` or `--resize`

- Set the resizing scale:
  - `-S` or `--scale`

Default scale: `2`

## Installation

**Using Cargo:**

```bash
cargo install imgr
```

**From Source:**

```bash
git clone https://github.com/shamxl/imgr.git
cd imgr
cargo build --release --path .
```

**Note:**

As I continue my journey with Rust, please pardon any imperfections in this program. They're all part of the learning process.
