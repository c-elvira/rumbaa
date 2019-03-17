use rumbaa;

#[cfg(test)]
mod tests {
	// Note this useful idiom: importing names from outer (for mod tests) scope.
	use super::*;

	#[test]
	fn test_theorem_without_label() {
		let filename = String::from("theorem_without_label.tex");
		let data_folder = String::from("tests/datas/");
		let _doc = rumbaa::texparser::parse_tex(&filename, &data_folder).unwrap();
	}

	#[test]
	fn test_equation_in_def() {
		let filename = String::from("equation_in_def.tex");
		let data_folder = String::from("tests/datas/");
		let doc = rumbaa::texparser::parse_tex(&filename, &data_folder).unwrap();

		// 1. test
		let label = String::from("th");
		let vec = doc.get_vec_dependences(&label).unwrap();

		assert_eq!(vec.len(), 1);
	}
}