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


    let pathbegins: &str = "./";
    let shreck = args[1].trim();
    let sepa = "-----------";

    println!("{}{}{}",pathbegins.black().bold().on_truecolor(79, 255, 112),shreck.black().bold().on_truecolor(79, 255, 112), sepa.truecolor(79, 255, 112).bold().on_truecolor(79, 255, 112));
    println!("{}", contents.truecolor( 79, 255, 112 ).bold());

    Ok(())
}

