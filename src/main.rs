// A simple Rust 1.14 implementation of cat using only std
// TODO: cat arguments: -A, -e, -s -t, -v
// TODO: move help and version text to separate files
// TODO: more comments
// TODO: "interactive cat" (like when you run cat with no args in bash)
// TODO: multiple files

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

static LABEL: &'static str = "[rustcat] ";
static VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        output_help();
    } else if args.len() > 10 {
        // arbitrary maximum amount of args
        println!("{}Too many arguments.", LABEL);
        return;
    } else {
        let (arg_list, help_mode, version_mode) = parse_args(&args);
        let filepath_arg = args.len() - 1;

        if help_mode {
            output_help();
        } else if version_mode {
            output_version();
        } else {
            readfile(&args[filepath_arg], arg_list);
        }
    }
}

fn parse_args(args: &Vec<String>) -> (Vec<&str>, bool, bool){
    // TODO: think of some other method to identify flags
    // TODO: make handling help_ and version_mode more logical
    let mut arg_list = Vec::new();

    for i  in 1..(args.len()) {
        if args[i] == "--help" {
            arg_list.push("help");
            break;
        }
        if args[i] == "--version" {
            arg_list.push("version");
            break;
        }

        if args[i] == "-n" || args[i] == "--number" {
            arg_list.push("numbers");
        } else if args[i] == "-E" || args[i] == "--show-ends" {
            arg_list.push("show-ends");
        } else if args[i] == "-b" || args[i] == "--number-nonblank" {
            arg_list.push("number-nonblank");
        } else if args[i] == "-T" || args[i] == "--show-tabs" {
            arg_list.push("show-tabs");
        }
    }

    let help_mode = arg_list.contains(&"help");
    let version_mode = arg_list.contains(&"version");
    (arg_list, help_mode, version_mode)
}

fn readfile(filepath: &str, arg_list: Vec<&str>) {
    // create path to file
    let path = Path::new(filepath);
    let display = path.display();

    // open file
    let mut file = match File::open(&path) {
        Err(why) => panic!("{}couldn't open {}: {}", LABEL, display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();

    // read file contents into string
    match file.read_to_string(&mut s) {
        Err(why) => panic!("{}couldn't read {}: {}", LABEL, display, why.description()),
        Ok(_) => {
            // TODO: probably refactor this bit
            //println!("{}{}:", LABEL, display);    // display filename

            let lines = s.lines();
            let mut whitespace_count = 0;
            let mut text;

            for (line_number, line_text) in lines.enumerate() {
                if arg_list.contains(&"show-tabs") {
                    text = line_text.replace("\t", "^I");
                } else {
                    text = line_text.replace("", "");
                }

                if arg_list.contains(&"numbers") && !arg_list.contains(&"number-nonblank") {
                    print!("{}\t", line_number + 1);
                }

                if arg_list.contains(&"number-nonblank") {
                    if line_text != "" {
                        print!("{}\t", line_number - whitespace_count + 1);
                    } else {
                        whitespace_count = whitespace_count + 1;
                    }
                }

                print!("{}", text);

                if arg_list.contains(&"show-ends") {
                    print!("$");
                }

                println!();
            }
        },
    };
}

// version info
fn output_version() {
    println!("\n[rustcat {}]", VERSION);
    println!("Copyright (C) 2016 Baerlabs");
    println!("License GPLv3: GNU GPL version 3 <http://gnu.org/licenses/gpl.html>");
    println!("rustcat is free to use, study, change, and redistribute.");
    println!("There is no warranty, to the extent permitted by law.");
    println!("\nWritten by yehnra");
    println!();
}

// help info
fn output_help() {
    println!("\n{}", LABEL);
    println!("Usage: rustcat [OPTION]... [FILE]");
    println!("Concatenate FILE to standard output\n");
    println!("  -b, --number-nonblank\t\tnumber nonempty output lines, overrides -n");
    println!("  -n, --number\t\t\tnumber all output lines");
    println!("  -E, --show-ends\t\tdisplay $ at end of each line");
    println!("  -T, --show-tabs\t\tdisplay TAB characters as ^I");
    println!("      --help\t\t\tdisplay this help and exit");
    println!("      --version\t\t\toutput version information and exit");
    println!();
}
