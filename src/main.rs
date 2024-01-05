use std::io::*;
use std::fs::File;
use std::env::args;
use colored::*;

fn main() {
    let _test = run();
}

fn run() -> Result<()> {

    let args: Vec<String> = args().collect();



    let mut file = File::open(args[1].trim())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;



    println!("{}", contents.truecolor( 79, 255, 112 ).bold());

    Ok(())
}

