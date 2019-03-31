extern crate tempfile;
extern crate regex;

use std::fs::{File,OpenOptions};
use std::io::{Error,Seek,SeekFrom};


pub fn wrap_and_preprocess(main_tex_filename: &String, out_filename: &String, folder: &String) -> Result<File, Error> {

	// 1. Create arXiv file
	//let arxiv_filename = assembles_filename(&"arxiv.tex".to_string(), folder);
	let mut arxiv_file = OpenOptions::new()
		.create(true)
		.read(true)
		.write(true)
		.append(false)
		.open(&out_filename).unwrap();

	// 2. open main file
	let path_to_main = String::clone(folder) + main_tex_filename;
	let main_tex_file = match File::open(&path_to_main) {
		Ok(f) => f,
		Err(_e) => {
			// File not found
			panic!("Unable to find {}", &path_to_main)
		}
	};

	// 3. recursively include tex in arxiv file
	process_text_internal::clean_and_copy_file(&mut arxiv_file, &main_tex_file, &folder);

	// 4. return
	arxiv_file.seek(SeekFrom::Start(0)).unwrap();
	return Ok(arxiv_file)
}

mod process_text_internal {

	use std::fs::{File};
	use std::io::{Write,BufReader};
	use std::io::prelude::*;
	use regex::Regex;

	pub fn clean_and_copy_file(dest_file: &mut File, input_file: &File, folder: &String) {
		// detect \begin{comment} ... \end{comment}
		let mut in_long_comment = false;

		// 1. Start loop
		let buf_reader = BufReader::new(input_file);
		'loop_lines: for (_num, l) in buf_reader.lines().enumerate() {
			let mut line = clean_line(&l.unwrap());

			if in_long_comment {
				if line.contains("\\end{comment}") == true {
					let split = line.split("\\end{comment}");
					let vec: Vec<&str> = split.collect();
					line = vec[1].to_string();
					in_long_comment = false;
				}
				else {
					continue 'loop_lines;
				}
			}
			else {
				if line.contains("\\begin{comment}") == true {
					let split = line.split("\\begin{comment}");
					let vec: Vec<&str> = split.collect();
					line = vec[0].to_string();

					in_long_comment = true;
				}
			}

			'loop_input: loop {
				match contain_input_file(&line) {
					Some(cmd_input) => {
						let latex_cmd = cmd_input.0;
						let filename = cmd_input.1;
						line = line.replace(&latex_cmd, "");

				   		let input_file_name = folder.to_owned() + &filename + &String::from(".tex");
						match File::open(&input_file_name) {
							Ok(f) => {
								clean_and_copy_file(dest_file, &f, &folder);
							},
							Err(_) => {
								// File not found
								println!("Warning: {} not found", filename);
							}
						};
					}
					None => {
						break 'loop_input
					}
				}
			};

			if !is_blank_line(&line) {
				writeln!(dest_file, "{}", line).unwrap();
			}
		}
	}

	fn clean_line(line: &String) -> String {

		let mut out: String;

		// 1.1 Check comments
		if line.contains("%!TEX") == true {
			// Keep line if it contains latexmk command
			out = line.clone();
		}
		else if line.contains("%") == true {
			// Contains a true comment. Try to remove it
			let split = line.split("%");
			let vec: Vec<&str> = split.collect();

			out = vec[0].to_string();
		}
		else {
			out = line.clone();
		}

		return out
	}

	fn contain_input_file(line: &String) -> Option<(String, String)> {

		let re = Regex::new(r"(\\input\{)(.*?)(\})").unwrap();

		   // Modify content (reaplace input)
		match re.captures(&line) {
			Some(cap) => {
				// cap[0].to_string(): full command
				// cap[2].to_string(): label
				return Some((cap[0].to_string(), cap[2].to_string()))
			},
			None => {
				return None
			},
		};
	}

	fn is_blank_line(line: &String) -> bool {
		let data = line.replace(" ", "");
		let data = data.replace("\t", "");
		if data == "" {
			return true
		}
		false
	}

	#[cfg(test)]
	mod tests {

	    #[test]
	    fn detect_blankline_1() {
	    	let line = String::from("");
	        assert_eq!(crate::preprocessing::process_text_internal::is_blank_line(&line), true);
	    }

	    #[test]
	    fn detect_blankline_2() {
	    	let line = String::from("   ");
	        assert_eq!(crate::preprocessing::process_text_internal::is_blank_line(&line), true);
	    }

	    #[test]
	    fn detect_blankline_3() {
	    	let line = String::from("	");
	        assert_eq!(crate::preprocessing::process_text_internal::is_blank_line(&line), true);
	    }
	}
}
