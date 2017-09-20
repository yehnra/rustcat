# rustcat
### A Rust 1.14 implementation of cat(1) using only std.
rustcat is still a work-in-progress (see TODO's in src/main.rs),
however, all of the following functionality works:

```
[rustcat]
Usage: rustcat [OPTION]... [FILE]
Concatenate FILE to standard output
  -b, --number-nonblank    number nonempty output lines, overrides -n
  -n, --number             number all output lines
  -E, --show-ends          display $ at end of each line
  -T, --show-tabs          display TAB characters as ^I
      --help               display this help and exit
	  --version            output version information and exit
```

## Examples:

```
# cargo
$ cargo run -- test_files/test.txt
$ cargo run -- -T test_files/tab_test.txt

# binary
$ rustcat test_files/test.txt
$ rustcat --show-ends test_files/test.csv
```

### Note:
rustcat currently only accepts multiple arguments using
the following syntax:

```
# cargo
$ cargo run -- -b -T test_files/tab_test.txt

# binary
$ rustcat -E -n test_files/test.txt
```

Support for multiple arguments using only one '-' will
be added eventually.

## Version and Licensing Info:

```
[rustcat 0.1.2]
Copyright (C) 2016 Baerlabs
License GPLv3: GNU GPL version 3 <http://gnu.org/licenses/gpl.html>
rustcat is free to use, study, change, and/or redistribute.
There is no warranty, to the extent permitted by law.

Written by yehnra
```
