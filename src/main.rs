mod texparser;
mod document;
mod texstruct;
mod textmpfile;
mod auxparser;
mod visualize;

#[macro_use]
extern crate clap;

use clap::{App};
use std::fs::{create_dir_all};

//use std::env;

fn main() {

	// 1. Processing input arguments
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

	let filename = String::from(matches.value_of("INPUT").unwrap());

	let data_folder = match matches.value_of("folder") {
		Some(f) => format_dir_name(String::from(f)),
		None 	=> String::from(""),
	};

	let aux_folder = match matches.value_of("auxiliary folder") {
		Some(f) => format_dir_name(String::from(f)),
		None 	=> String::from(""),
	};

	let output_folder = match matches.value_of("output") {
		Some(f) => format_dir_name(String::from(f)),
		None 	=> String::from(""),
	};
	match create_dir_all(&output_folder) {
		Ok(_) => (),
		Err(_) => panic!("A problem occurs with argument -o:\n{}\n
			\tIs it a valid directory filename", output_folder)
	};

	let verbose = matches.occurrences_of("verbose");

	// 2. 
	if verbose >= 1	{
	    println!("Processing file {}:", filename);
	}

    let mut doc = match texparser::parse_tex(&filename, &data_folder) {
    	Ok(d) => d,
    	Err(e) => panic!("An errror had occured while parsing tex file\n{}", e),
    };

    match auxparser::parse_aux(&filename, &aux_folder, &mut doc, &verbose) {
    	Ok(()) => (),
    	Err(e) => println!("an error occurs while parsing aux file\n{}", e),
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

fn format_dir_name(dir: String) -> String {
	
	if dir.ends_with("/") == false {
		return format!("{}/", dir)
	}

	dir
}