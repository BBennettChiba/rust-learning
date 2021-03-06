use std::{env, process};

use cli::{run, Config};

fn main() {
    // let args: Vec<String> = env::args().collect();
    let args = env::args();
    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
