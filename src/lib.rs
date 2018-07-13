use std::fs::File;
use std::process;

pub fn parse_args(args : &[String]) -> (&str, &[String]) {
    if args.len() < 3 {
        println!("Error: Not enough arguments");
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
    let f = match File::open(file) {
        Ok(fd) => fd,
        Err(_) => {
            println!("Error: unable to open file {}", file);
            return;
        }
    };
}
