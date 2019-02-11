pub mod texparser;
mod texstruct;

use std::env;

fn main() {
    println!("Reading command line arguments");

	let args: Vec<String> = env::args().collect();
    let filename = 
		if args.len() > 1 {
			String::clone(&args[1])
		} else {
			String::from("texdata/main.tex")
		};

    println!("Processing file {}:\n\n", filename);
    let doc = texparser::parse_tex(filename);

    println!("{}", doc.print());
}