<h1>Rustcat</h1>

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

This is still a work-in-progress (see TODO's in src/main.rs).
rustcat is a Rust 1.14 implementation of cat(1) using only std.

