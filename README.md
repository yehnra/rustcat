<h1>rustcat</h1>
<h4>rustcat is a Rust 1.14 implementation of cat(1) using only std.</h4>
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

<h3>Examples:</h3>
```
# using cargo
$ cargo run -- test_files/test.txt
$ cargo run -- -T test_files/tab_test.txt

# using binary
$ rustcat test_files/test.txt
$ rustcat --show-ends test_files/test.csv
```

<h4>Note:</h4>
rustcat currently only accepts multiple arguments using
the following syntax:
```
# using cargo
$ cargo run -- -b -T test_files/tab_test.txt

# using binary
$ rustcat -E -n test_files/test.txt
```
Support for multiple arguments using only one '-' will
be added eventually.

<h3>Version and licensing info:</h3>

```
[rustcat 0.1.2]
Copyright (C) 2016 Baerlabs
License GPLv3: GNU GPL version 3 <http://gnu.org/licenses/gpl.html>
rustcat is free to use, study, change, and redistribute.
There is no warranty, to the extent permitted by law.

Written by yehnra
```
