extern crate gsp;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, files) = parse_args(&args);

    search(query, files);
}


