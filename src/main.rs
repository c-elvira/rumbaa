mod texparser;
mod document;
mod texstruct;
mod preprocessing;
mod auxparser;
mod visualize;

#[macro_use]
extern crate clap;

use clap::{App};
use std::fs::{create_dir_all,remove_file};

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

	let save_to_arxiv = matches.is_present("arxiv");
	let arxiv_filename = data_folder.clone() + &String::from("arxiv.tex");

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
	
	// 1. Wrap all files in one
	//	 + Remove comments
	let clean_file = match preprocessing::wrap_and_preprocess(&filename, &arxiv_filename, &data_folder) {
		Ok(f) => f,
		Err(_e) => panic!("{:?}", _e),
	};

	// 2. Parse latex
    let mut doc = match texparser::parse_tex(&clean_file, &filename, &data_folder) {
    	Ok(d) => d,
    	Err(e) => panic!("An errror had occured while parsing tex file\n{}", e),
    };

   	// 3. Parse aux
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

	// 3.2 delete arxiv file if necessary
	if !save_to_arxiv {
		delete_file_if_exist(&arxiv_filename);
	}

	// 4. Visualization
    visualize::visualize(&doc, &output_folder)
    	.expect("Something went wrong when exporting tex document");
}

fn format_dir_name(dir: String) -> String {
	
	if dir.ends_with("/") == false {
		return format!("{}/", dir)
	}

	dir
}

fn delete_file_if_exist(filename: &String) {
	match remove_file(&filename) {
		Ok(()) => return,
		Err(_e) => return,
	};
}