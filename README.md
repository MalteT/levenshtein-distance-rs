```
distance 0.1
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
        --columnsep <SEP>       Seperator between columns [default:  ]
        --from-file <FILE>      Read filenames from FILE
        --linesep <SEP>         EOL seperator [default:
                                ]
    -s, --string <STRING>...    Compare the given string

ARGS:
    <FILE>...    Compare the given file's content
```
