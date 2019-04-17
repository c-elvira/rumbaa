//mod texparser;
mod texparser;
mod envparser;
mod document;
mod texstruct;
mod preprocessing;
mod auxparser;
mod visualize2;

#[macro_use] extern crate clap;
#[macro_use] extern crate log;

extern crate simplelog;

use simplelog::*;
use clap::{App};
use std::fs::{File,create_dir_all,remove_file};

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
	if save_to_arxiv{
	    info!("Saving clean arXiv version");
	}
	let arxiv_filename = data_folder.clone() + &String::from("arxiv.tex");

	match create_dir_all(&output_folder) {
		Ok(_) => (),
		Err(_) => panic!("A problem occurs with argument -o:\n{}\n
			\tIs it a valid directory filename", output_folder)
	};

	let verbose = matches.occurrences_of("verbose");

	// 0. Create logger
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Trace, Config::default()).unwrap(),
            WriteLogger::new(LevelFilter::Trace,
            	Config::default(),
            	File::create(output_folder.clone() + &"rumbaa.log".to_string()).unwrap()),
        ]
    ).unwrap();

	//
    trace!("Processing file {}:", filename);
	
	// 1. Wrap all files in one
	//	 + Remove comments
    trace!("1. Preprocessing");
	delete_file_if_exist(&arxiv_filename);
	let clean_file = match preprocessing::wrap_and_preprocess(&filename, &arxiv_filename, &data_folder) {
		Ok(f) => f,
		Err(_e) => panic!("{:?}", _e),
	};

	// 2. Parse latex
    trace!("2. parsing Latex");
    //let mut doc = match texparser::parse_tex(&clean_file, &filename, &data_folder) {
   	let mut doc = match texparser::texparser::parse_tex(&clean_file, &filename) {
    	Ok(d) => d,
    	Err(e) => panic!("An errror had occured while parsing tex file\n{}", e),
    };

   	// 3. Parse aux
    trace!("3. parsing aux");
    match auxparser::parse_aux(&filename, &aux_folder, &mut doc, &verbose) {
    	Ok(()) => (),
    	Err(e) => println!("an error occurs while parsing aux file\n{}", e),
    };

	// 3.2 delete arxiv file if necessary
	if !save_to_arxiv {
		delete_file_if_exist(&arxiv_filename);
	}

	// 4. Visualization
    trace!("4. Visualization");
    visualize2::visualize(&doc, &output_folder)
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