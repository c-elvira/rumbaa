mod texparser;
mod texstruct;
mod textmpfile;
mod visualize;

#[macro_use]
extern crate clap;

use clap::{App};

//use std::env;

fn main() {

	// Processing inputs
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

	let filename = String::from(matches.value_of("INPUT").unwrap());

	let folder = match matches.value_of("folder") {
		Some(f) => String::from(f),
		None 	=> String::from(""),
	};

	let _auxfolder = match matches.value_of("auxiliary folder") {
		Some(f) => String::from(f),
		None 	=> String::from(""),
	};

	let output_folder = match matches.value_of("output") {
		Some(f) => String::from(f),
		None 	=> String::from(""),
	};

    println!("Processing file {}:\n\n", filename);
    let doc = texparser::parse_tex(&filename, &folder).unwrap();

    println!("{}", doc.print());
    
    visualize::visualize(&doc, &output_folder)
    	.expect("Something went wrong when exporting tex document");
}
