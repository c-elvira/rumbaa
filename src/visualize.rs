use crate::texstruct::{Document};

#[allow(unused_imports)]
use std::fs::{File,OpenOptions,read_to_string,remove_file};
#[allow(unused_imports)]
use std::io::{Write, Read, copy,Seek,Error,SeekFrom};


pub fn visualize(doc: &Document) -> Result<(), Error> {

	let outdir = String::from("out/");
	export_to_json(doc, &outdir)?;

	Ok(())
}


fn export_to_json(doc: &Document, outdir: &String) -> Result<(), Error> {
	// Relies on https://bl.ocks.org/heybignick/3faf257bbbbc7743bb72310d03b86ee8

		// Remove if file exists
	remove_file(outdir.to_owned() + "texstruct.json").unwrap();

		// Then creates it
	let mut jsonfile = OpenOptions::new()
		.read(true)
		.append(true)
		.create(true)
		.open(outdir.to_owned() + "texstruct.json")
		?;
	
	writeln!(
		jsonfile,
		"{{"
		).unwrap();

	export_json_nodes(doc, &mut jsonfile)?;

	writeln!(
		jsonfile,
		"\t,"
		).unwrap();

	export_json_links(doc, &mut jsonfile)?;

	writeln!(
		jsonfile,
		"}}"
		).unwrap();

	Ok(())
}

fn export_json_nodes(doc: &Document, jsonfile: &mut File) -> Result<(), Error> {

	writeln!(
		jsonfile,
		"\t\"nodes\": ["
		).unwrap();

	let mut i = 0;
	for key in doc.keys() {
		// {"id": "Myriel", "group": 1},
		if i == 0 {
			writeln!(
				jsonfile,
				"\t\t{{\"id\": \"{}\", \"group\": 1}}", key
			).unwrap();
		}
		else {
			writeln!(
				jsonfile,
				"\t\t, {{\"id\": \"{}\", \"group\": 1}}", key
			).unwrap();
		}

		i += 1;
	}

	writeln!(
		jsonfile,
		"\t]"
		).unwrap();

	Ok(())
}


fn export_json_links(doc: &Document, jsonfile: &mut File) -> Result<(), Error> {

	writeln!(
		jsonfile,
		"\t\"links\": ["
		).unwrap();

	let mut i = 0;
	for key in doc.keys() {
		// {"source": "Napoleon", "target": "Myriel", "value": 1},
		let texstruct = doc.get(key.to_string());
		let proof = match texstruct.get_proof() {
			Some(expr) => expr,
			None => continue,
		 };

		for j in 0..proof.get_nblinks() {
			if i == 0 {
				writeln!(
					jsonfile,
					"\t\t{{\"source\": \"{}\", \"target\":\"{}\", \"value\": 1}}", key, proof.get_link(j),
				).unwrap();
			}
			else {
				writeln!(
					jsonfile,
					"\t\t, {{\"source\": \"{}\", \"target\":\"{}\", \"value\": 2}}", key, proof.get_link(j),
				).unwrap();			}

			i += 1;
		}
	}
	
	writeln!(
		jsonfile,
		"\t]"
		).unwrap();

	Ok(())
}