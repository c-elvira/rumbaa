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

		let _doc = rumbaa::texparser::parse_tex(&main_file, &filename, &data_folder).unwrap();
	}

	#[test]
	fn test_equation_in_def() {
		let filename = String::from("equation_in_def.tex");
		let data_folder = String::from("tests/datas/");

		let clean_file = rumbaa::preprocessing::wrap_and_preprocess(&filename, &data_folder).unwrap();
		let doc = rumbaa::texparser::parse_tex(&clean_file, &filename, &data_folder).unwrap();

		// 1. test
		let label = String::from("th");
		let vec = doc.get_vec_dependences(&label).unwrap();

		// 2. Delete file
		match remove_file("tests/datas/rtex_tmp.tex") {
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

		let clean_file = rumbaa::preprocessing::wrap_and_preprocess(&filename, &data_folder).unwrap();
		let doc = rumbaa::texparser::parse_tex(&clean_file, &filename, &data_folder).unwrap();

		// 2. Delete file
		match remove_file("tests/datas/rtex_tmp.tex") {
			Ok(()) => (),
			Err(_) => (),
		};

		// 3. Test
		assert_eq!(doc.contains_key(&"def".to_string()), true);
		assert_eq!(doc.contains_key(&"def:commented".to_string()), false);
		assert_eq!(doc.contains_key(&"th:commented".to_string()), false);
	}
}