# imgr
image to ascii converter written in rust

# Usage

```command
Usage: imgr [OPTIONS] <FILENAME>

Arguments:
  <FILENAME>
          path of the image

Options:
  -s, --style <STYLE>
          [default: ascii]

          Possible values:
          - ascii:   print as ascii chars
          - block:   print as block
          - braille: print as braille - ! Experimental (use with color on)

  -S, --scale <SCALE>
          to set the scale of image

          [default: 5]

  -c, --colored
          to set the output color

  -r, --resize
          to set wheather the image to be resized or not

  -b, --block <BLOCK>
          to set the char of block style

          [default: █]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

# Installation
```command
$ git clone https://github.com/shamxl/imgr.git
$ cd imgr
$ cargo build --release
$ ..
```

#### Note
im still learning rust so there will be some mistakes in this program, sorry
