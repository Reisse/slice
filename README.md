# slice
`slice` allows to filter text data in a manner similiar to Python's [slicing](https://docs.python.org/3/reference/expressions.html?highlight=slice#slicings) (informal description with examples [here](https://docs.python.org/3/tutorial/introduction.html#strings)).

`slice` is designed to perform queries like "extract lines 12 to 21", "show first 10 lines", "show first 4 lines from last 10", "show last 6 lines".

In many cases `slice` can replace both `head` and `tail`.

## Installation
Clone this repository:
```
$ git clone https://github.com/Reisse/slice.git
```
Then, install with `cargo`:
```
$ cargo install --path .
```

## Usage
```
$ slice [OPTION]... [FILE]
```
Print slice from FILE to standard output. When slice is not specified, print whole file to standard output.

## Options
|Short option|Long option|Arguments|Description                         |
|------------|-----------|---------|------------------------------------|
|-s          |--slice    |BEGIN:END|specify slice to print              |
|-h          |--help     |         |display this help and exit          |
|-v          |--version  |         |output version information and exit |
|            |--         |         |end of options                      |

BEGIN and END may be any combination of positive (denoting position from the beginning) or negative (denoting position from the end) numbers.

## Example
```
$ cat test
1
2
3
4
5
$ slice -s 0:2 test
1
2
$ slice -s -1:5 test
5
$ slice -s 2:-1 test
3
4
$ slice -s -5:-3 test
1
2
```

## Caveats
- Both LF and CRLF are recognized as newline characters, but newlines are not preserved and are always replaced with LF in the output.
- Last line of the output will always end with LF.

## License
See LICENSE file.
