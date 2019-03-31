use rumbaa;

use std::fs::File;
use std::io::{BufReader};
use std::fs::{remove_file,OpenOptions};
use std::io::prelude::*;

#[cfg(test)]
mod tests_arxiv {
	// Note this useful idiom: importing names from outer (for mod tests) scope.
	use super::*;

	#[test]
	fn detect_blank_lines() {
		let filename = String::from("1_arxiv_blankLines.tex");
		let filename_expected = String::from("1_arxiv_blankLines_expected.tex");
		let data_folder = String::from("tests/datas/arxiv_data/");
		let tmp_file_name = String::from("tmp_detect_blank_lines.tex");

		// 1. preprocess file
		let clean_file = rumbaa::preprocessing::wrap_and_preprocess(&filename, &tmp_file_name, &data_folder).unwrap();

		// 2. Read contents

			// 2.1 expected
		let mut content_expected = String::new();
		let file_expected = OpenOptions::new()
			.read(true)
			.write(false)
			.open(data_folder.clone() + &filename_expected).unwrap();
		BufReader::new(file_expected).read_to_string(&mut content_expected).unwrap();

			// 2.2 clean one
		let mut content_out = String::new();
		BufReader::new(clean_file).read_to_string(&mut content_out).unwrap();


		// 3. Delete tmp file
		match remove_file(tmp_file_name) {
			Ok(()) => (),
			Err(_) => (),
		};

		// 3. Test
		assert_eq!(content_out, content_expected);
	}
}