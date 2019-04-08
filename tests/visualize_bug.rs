use rumbaa;

//use std::fs::File;
use std::io::{BufReader};
use std::fs::{remove_file,OpenOptions};
use std::io::prelude::*;

#[cfg(test)]
mod tests_arxiv {
	// Note this useful idiom: importing names from outer (for mod tests) scope.
	use super::*;

	#[test]
	fn test_remove_comments() {
		let filename = String::from("1_vizu_call_proven_th_bug.tex");
		let data_folder = String::from("tests/datas/vizualize_data/");
		let tmp_file_name = String::from("tmp_1.tex");

		let clean_file = rumbaa::preprocessing::wrap_and_preprocess(&filename, &tmp_file_name, &data_folder).unwrap();
		let doc = rumbaa::texparser::texparser::parse_tex(&clean_file, &filename).unwrap();
	    rumbaa::visualize::visualize(&doc, &data_folder)
    		.expect("Something went wrong when exporting tex document");

    	// 1. Extract json
		let mut content_json = String::new();
		let file_json = OpenOptions::new()
			.read(true)
			.write(false)
			.open(data_folder.clone() + &String::from("texstruct.json")).unwrap();
		BufReader::new(file_json).read_to_string(&mut content_json).unwrap();

		// 2. Delete file
		match remove_file(tmp_file_name) {
			Ok(()) => (),
			Err(_) => (),
		};
		match remove_file(data_folder.clone() + &String::from("texstruct.json")) {
			Ok(()) => (),
			Err(_) => (),
		};
		match remove_file(data_folder.clone() + &String::from("index.html")) {
			Ok(()) => (),
			Err(_) => (),
		};

		// 3. Test
		let line_expected = "\"source\": \"th\", \"target\":\"def\"";
		assert_eq!(content_json.contains(line_expected), true);
	}
}