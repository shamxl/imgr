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

**Output**

- Write the output to a file instead of printing to terminal
  - `-o <path to file>` or `--output <path to file>`


## Installation

**Using Cargo:**

```bash
cargo install imgr
```

**From Source:**

```bash
git clone https://github.com/shamxl/imgr.git
cd imgr
cargo build --release 
```

**Note:**

As I continue my journey with Rust, please pardon any imperfections in this program. They're all part of the learning process.


## Frequently Asked Questions

**Q: I installed `imgr` using Cargo, but I can't find the binary. Where is it?**

A: By default, Cargo installs binaries in the `.cargo` directory. To make `imgr` easily accessible, consider adding the Cargo bin directory to your system's `PATH`. You can find the location of the Cargo bin directory by running `cargo install --help` and looking for the "bin" section.

**Q: How can I add the Cargo bin directory to my system's `PATH`?**

A: On Unix-based systems (Linux, macOS), you can add the following line to your shell profile file (e.g., `.bashrc`, `.zshrc`):

```bash
export PATH="$PATH:$HOME/.cargo/bin"
```

On Windows, you can add the Cargo bin directory to the system environment variables.

After making these changes, restart your terminal, and you should be able to run `imgr` from anywhere in the command line.
