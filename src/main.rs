mod texparser;
mod document;
mod texstruct;
mod textmpfile;
mod auxparser;
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

	let aux_folder = match matches.value_of("auxiliary folder") {
		Some(f) => String::from(f),
		None 	=> String::from(""),
	};

	let output_folder = match matches.value_of("output") {
		Some(f) => String::from(f),
		None 	=> String::from(""),
	};

	let verbose = matches.occurrences_of("verbose");

	if verbose >= 1	{
	    println!("Processing file {}:", filename);
	}

    let mut doc = match texparser::parse_tex(&filename, &folder) {
    	Ok(d) => d,
    	Err(e) => panic!("An errror had occured while parsing tex file\n{}", e),
    };
    
    match auxparser::parse_aux(&filename, &aux_folder, &mut doc, &verbose) {
    	Ok(()) => (),
    	Err(_e) => println!("an error occurs while parsing aux file\n {}", _e),
    };

    if verbose >= 2 {
	    println!("{}", doc.print());
	}

	if verbose >= 1	{
	    println!("Exporting tex structure");
	}

    visualize::visualize(&doc, &output_folder)
    	.expect("Something went wrong when exporting tex document");
}
