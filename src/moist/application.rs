use std::path::Path;
use std::env;
use crate::moist::hydrate::hydrate;

pub fn init() {
    // TODO Add loggers here
    let args: Vec<String> = env::args().collect();
    parse(&args);
}

fn parse(args: &Vec<String>) {
    if !args[1].eq("hydrate") {
        println!("Invalid input format");
        return;
    }

    let input_dir = Path::new(&args[2]);
    hydrate(input_dir);
}