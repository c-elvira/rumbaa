extern crate regex;

use std::fs::{File};
use std::io::{BufReader};
use std::io::{Error};
use std::io::prelude::*;

use regex::Regex;
use crate::document::{Document};

/**
 * @brief [brief description]
 * @details Implement the state machine
 * 
 * @param e [description]
 * @return [description]
 */
pub fn parse_aux(filename: &String, folder: &String, mut doc: &mut Document, verbose: &u64) -> std::result::Result<(), Error> {
	// 1. get aux path
	let aux_path = get_aux_path(filename, folder);

	// 2. open aux file
	if *verbose >= 1 {
		println!("parsing {}", aux_path);
	}
	let aux_file = File::open(aux_path)?;

	// 3. get content
	let buf_reader = BufReader::new(aux_file);
	//buf_reader.read_to_string(&mut content).unwrap();

	// 4. Process line per line
	for (_num, line) in buf_reader.lines().enumerate() {
    	let mut l = line.unwrap();

		if l.starts_with("\\newlabel") & l.contains("@cref") == false {
			l = l.replace("{{", "{").replace("}}", "}");
			
			match process_line(&l, &mut doc) {
				Ok(()) => (),
				Err(e) => {
					println!("Error while processing aux");
					println!("Line: {:?}", l);
					println!("{:?}", e);
				}
			};
		}
	}

	Ok(())
}

fn get_aux_path(filename: &String, folder: &String) -> String {
	
    let split = filename.split(".");
    let vec: Vec<&str> = split.collect();
    
    let radical = vec[0];

    let result = String::clone(folder) + radical + ".aux";
    result
}

fn process_line(line: &String,  doc: &mut Document) -> std::result::Result<(), Error> {

	let mut nb_match = 0;
	let mut strlabel = String::from("");
	let mut label_number: i32;
	let mut label_page: i32;
	let mut name: String;

	let regex = Regex::new(r"\{(.*?)\}").unwrap();
	for cap in regex.captures_iter(line) {
		match nb_match {
			0 => {
				strlabel = String::from(cap[1].to_string());
				if doc.contains_key(&strlabel) == false {
					break;
				}
			},
			// Simple form
			// \newlabel{def:label1}{{1}{1}}
			1 => {
				label_number = match cap[1].parse::<i32>() {
					Ok(i) => i,
					Err(_) => break,
				};
				doc.set_label_number(&strlabel, label_number);
			},
			2 => {
				label_page = match cap[1].parse::<i32>() {
					Ok(i) => i,
					Err(_) => break,
				};
				doc.set_page(&strlabel, label_page);
			},
			// More complicated form
			// \newlabel{th:contrib:cmf_uniformRecov_1D}{{2}{15}{}{theorem.3.2}{}}
			3 => {
				// Not implemented
			},
			4 => {
				name = String::from(cap[1].to_string());
				doc.set_name(&strlabel, name);
			},
			5 => {
				// Not implemented
			},
			_ => {
				// Not implemented yet
				break;
			},
		}

		nb_match += 1;
	}

	Ok(())
}
