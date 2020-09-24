## Installation

With Rust installed, simply clone this repository and install or compile it using `cargo`. Note that `cargo` will install the program
to `~/.cargo/bin` on Linux machines and to some other place on Windows and Mac. At least on Linux the `PATH` variable will need some adjustment.

```shell
> git clone https://github.com/MalteT/levenshtein-distance-rs
> cd levenshtein-distance-rs
> cargo install --path .   # for the installation
> cargo run --release      # for compilation only
```

For installation only, this can be compressed to
```shell
> cargo install --git https://github.com/MalteT/levenshtein-distance-rs
```

## Usage

In the simplest form just run:
```shell
> distance FileA.txt FileB.txt
13 FileA.txt FileB.txt
```
This will result in a triple containing the levenshtein distance for the following two paths.

If more than two filesnames are supplied all possible combinations will be evaluated:
```shell
> distance FileA.txt FileB.txt FileC.txt
13 FileA.txt FileB.txt
0  FileA.txt FileC.txt
13 FileB.txt FileC.txt
```

Paths can also be read from a file using this method:
```shell
> distance --from-file files.txt # line-separated paths inside
0 FileA.txt FileB.txt
0 FileA.txt FileC.txt
0 FileA.txt FileD.txt
0 FileB.txt FileC.txt
0 FileB.txt FileD.txt
0 FileC.txt FileD.txt
```

Last but not least, strings can be supplied instead of files
```shell
> distance -s "Hello World" -s "Hello world."
2 <string0> <string1>
```

### `distance --help`
```
distance 1.0.0
Malte Tammena
Calculate edit distances

USAGE:
    distance [FLAGS] [OPTIONS] [--] [FILE]...

FLAGS:
    -h, --help                Prints help information
    -r, --relative            Calculate distance relative to the amount of characters of the longest string
    -t, --trim-whitespaces    Trim whitespaces and and empty lines at start and end of strings before comparision
    -V, --version             Prints version information

OPTIONS:
        --columnsep <SEP>       Seperator between output columns [default:  ]
        --from-file <FILE>      Read filenames from FILE. The file contains line-seperated paths. Whitespaces will be
                                trimmed from the start and end of each line.
        --linesep <SEP>         Output EOL seperator [default:\n]
    -s, --string <STRING>...    Compare the given string

ARGS:
    <FILE>...    Compare the given file's content
```
