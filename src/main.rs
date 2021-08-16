use std::env;
use std::process;
use spagreppi::Config;


fn main() {
    let args : Vec<String>  = env::args().collect();
    let config : Config= Config::new(&args).unwrap_or_else(|err| {
        println!("Welcome to spagreppi. {}", err);
        process::exit(1);
    });

    // run(config);
    if let Err(e) =  spagreppi::run(config) { // doesn’t return a value that we want to unwrap
        println!("error reading the file: {}", e); //we only care about detecting an error
        process::exit(1);
    }
   
}