use crate::document::{Document};

#[allow(unused_imports)]
use std::fs::{File,OpenOptions,read_to_string,remove_file};
#[allow(unused_imports)]
use std::io::{Write, Read, copy,Seek,Error,SeekFrom};


pub fn visualize(doc: &Document, out_dir: &String) -> Result<(), Error> {

	export_to_json(doc, &out_dir)?;

	// 2. create html page
	let html_text = include_str!("ressources/index2.html");
	let mut html_file = OpenOptions::new()
		.read(true)
		.write(true)
		.create(true)
		.open(out_dir.to_owned() + "index.html")
		?;

	writeln!(html_file,"{}", html_text).unwrap();

	Ok(())
}


fn export_to_json(doc: &Document, outdir: &String) -> Result<(), Error> {
	// Relies on https://bl.ocks.org/heybignick/3faf257bbbbc7743bb72310d03b86ee8

		// Remove if file exists
	match remove_file(outdir.to_owned() + "texstruct.json") {
		Ok(_) => (),  // file existed, it has been deleted
		Err(_) => (), // file did not exist, nothing happened

	};

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

		let name = doc.get_name_from_key(key).unwrap();
		let group = doc.get_group_from_key(key).unwrap();
		// {"id": "Myriel", "group": 1},
		if i == 0 {
			writeln!(
				jsonfile,
				"\t\t{{\"id\": \"{}\", \"group\": {}}}", name, group
			).unwrap();
		}
		else {
			writeln!(
				jsonfile,
				"\t\t, {{\"id\": \"{}\", \"group\": {}}}", name, group
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
	'loop_source: for key in doc.keys() {
		// Unwrap is safe since key comes from keys()
		let vec_dependences = doc.get_vec_dependences(key).unwrap();

		// Unwrap is safe since key comes from doc.keys()
		let name1 = doc.get_name_from_key(key).unwrap();
		for elem in vec_dependences {
			// {"source": "Napoleon", "target": "Myriel", "value": 1},
			// Unwrap should be safe since doc.structs_contain_label return true
			let name2 = doc.get_name_from_key(&elem).unwrap();
			if name1 == name2 {
				continue 'loop_source;
			}

			if i == 0 {
				writeln!(
					jsonfile,
					"\t\t{{\"source\": \"{}\", \"target\":\"{}\", \"value\": 1}}", name1, name2,
				).unwrap();
			}
			else {
				writeln!(
					jsonfile,
					"\t\t, {{\"source\": \"{}\", \"target\":\"{}\", \"value\": 1}}", name1, name2,
				).unwrap();
			}

			i += 1;
		}
	}
	
	writeln!(
		jsonfile,
		"\t]"
		).unwrap();

	Ok(())
}