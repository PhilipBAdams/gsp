use std::fs::File;
use std::process;
use std::io::prelude::*;

pub fn parse_args(args : &[String]) -> (&str, &[String]) {
    if args.len() < 3 {
        eprintln!("Error: Not enough arguments");
        process::exit(1);
    }
    (&args[1], &args[2..])
}

pub fn search(query : &str, files : &[String])
{
    for file in files {
        search_in_file(query, &file);
    }
}

fn search_in_file(query : &str, file : &str)
{
    let mut f = match File::open(file) {
        Ok(fd) => fd,
        Err(_) => {
            eprintln!("Error: unable to open file {}", file);
            return;
        }
    };

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Error: something went wrong reading the file: {}, file");

    for line in contents.lines() {
        if line.contains(query) {
            println!("{}", line);
        }
    }
}
