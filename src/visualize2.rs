use crate::document::{Document};

use std::fs::{File,OpenOptions};
use std::io::{Write,Error};

pub fn visualize(doc: &Document, out_dir: &String) -> Result<(), Error> {

	// 1a. read html page
	let html_text = include_str!("ressources/index3.html");

	// 1b. create output html page
	let mut output_file = OpenOptions::new()
		.read(true)
		.write(true)
		.create(true)
		.append(false)
		.open(out_dir.to_owned() + "graph_theorem.html")
		?;

	// 2. Start copying
	for line in html_text.split("\n") {

		if line != "#LINKS" {
			// In this case, simply copy file
			writeln!(output_file,"{}", line).unwrap();
		}
		else {
			_export_json_links(doc, &mut output_file)?;
		}
	}

	Ok(())
}


fn _export_json_links(doc: &Document, jsonfile: &mut File) -> Result<(), Error> {

	writeln!(
		jsonfile,
		"var links = ["
		).unwrap();

	'loop_source: for key in doc.keys() {
		// Unwrap is safe since key comes from keys()
		let vec_dependences = doc.get_vec_dependences(key).unwrap();

		// Unwrap is safe since key comes from doc.keys()
		let name1 = doc.get_name_from_key(key).unwrap();

		// Adding first self reference to create node if unused
		let self_ref = format!("\t\t{{\"source\": \"{}\", \"target\":\"{}\", \"type\": 1}}, ", name1, name1);
		writeln!(
			jsonfile,
			"{}", self_ref
		).unwrap();

		'loop_dep: for elem in vec_dependences {
			// {"source": "Napoleon", "target": "Myriel", "value": 1},
			// Unwrap should be safe since doc.structs_contain_label return true
			let name2 = match doc.get_name_from_key(&elem) {
				Some(s) => s,
				None => continue 'loop_dep,
			};

			writeln!(
				jsonfile,
				"\t\t{{\"source\": \"{}\", \"target\":\"{}\", \"type\": 1}},", name1, name2,
			).unwrap();
		}
	}
	
	writeln!(
		jsonfile,
		"]"
		).unwrap();

	Ok(())
}