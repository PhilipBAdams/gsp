extern crate gsp;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, files) = gsp::parse_args(&args);

    gsp::search(query, files);
}


