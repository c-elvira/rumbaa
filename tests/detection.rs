use rumbaa;

use std::fs::{OpenOptions,remove_file};

#[cfg(test)]
mod tests {
	// Note this useful idiom: importing names from outer (for mod tests) scope.
	use super::*;

	#[test]
	fn test_theorem_without_label() {
		let filename = String::from("theorem_without_label.tex");
		let data_folder = String::from("tests/datas/");

		let main_file = OpenOptions::new()
			.create(false)
        	.read(true)
        	.write(false)
        	.open(data_folder.clone() + &filename)
        	.unwrap();

		let _doc = rumbaa::texparser2::texparser::parse_tex(&main_file, &filename).unwrap();
	}

	#[test]
	fn test_equation_in_def() {
		let filename = String::from("equation_in_def.tex");
		let data_folder = String::from("tests/datas/");
		let tmp_file_name = String::from("tmp_eq_in_def.tex");

		let clean_file = rumbaa::preprocessing::wrap_and_preprocess(&filename, &tmp_file_name, &data_folder).unwrap();
		let doc = rumbaa::texparser2::texparser::parse_tex(&clean_file, &filename).unwrap();

		// 1. test
		let label = String::from("th");
		let vec = doc.get_vec_dependences(&label).unwrap();

		// 2. Delete file
		match remove_file(tmp_file_name) {
			Ok(()) => (),
			Err(_) => (),
		};

		// 3. Test
		assert_eq!(vec.len(), 1);
	}

	#[test]
	fn test_remove_comments() {
		let filename = String::from("remove_comments.tex");
		let data_folder = String::from("tests/datas/");
		let tmp_file_name = String::from("tmp_remove_comments.tex");

		let clean_file = rumbaa::preprocessing::wrap_and_preprocess(&filename, &tmp_file_name, &data_folder).unwrap();
		let doc = rumbaa::texparser2::texparser::parse_tex(&clean_file, &filename).unwrap();

		// 2. Delete file
		match remove_file(tmp_file_name) {
			Ok(()) => (),
			Err(_) => (),
		};

		// 3. Test
		assert_eq!(doc.contains_key(&"def".to_string()), true);
		assert_eq!(doc.contains_key(&"def:commented".to_string()), false);
		assert_eq!(doc.contains_key(&"th:commented".to_string()), false);
	}

	/**
	 * @brief commented \input causes crash
	 * @details Associated to Issue #11
	 * @return
	 */
	#[test]
	fn test_input_in_comment() {
		let filename = String::from("input_in_comment.tex");
		let data_folder = String::from("tests/datas/");
		let tmp_file_name = data_folder.to_owned() + &String::from("tmp_input_in_comment.tex");

		let clean_file = rumbaa::preprocessing::wrap_and_preprocess(&filename, &tmp_file_name, &data_folder).unwrap();
		let doc = rumbaa::texparser2::texparser::parse_tex(&clean_file, &filename).unwrap();

		// 2. Delete file
		match remove_file(tmp_file_name) {
			Ok(()) => (),
			Err(_) => (),
		};

		// 3. Test
		assert_eq!(doc.contains_key(&"th".to_string()), true);
	}
}