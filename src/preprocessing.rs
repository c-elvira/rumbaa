extern crate tempfile;
extern crate regex;

use std::fs::{File,OpenOptions,remove_file};
use std::io::{Write,BufReader,Seek,Error,SeekFrom};
use std::io::prelude::*;
use regex::Regex;


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
	_clean_and_copy_file(&mut arxiv_file, &main_tex_file, &folder);

	// 4. return
	return Ok(arxiv_file)
}

fn _clean_and_copy_file(dest_file: &mut File, input_file: &File, folder: &String) {
	// detect \begin{comment} ... \end{comment}
	let mut in_long_comment = false;

	// 1. Start loop
	let buf_reader = BufReader::new(input_file);
	'loop_lines: for (_num, l) in buf_reader.lines().enumerate() {
		let mut line = _clean_line(&l.unwrap());

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
			match _contain_input_file(&line) {
				Some(cmd_input) => {
					let latex_cmd = cmd_input.0;
					let filename = cmd_input.1;
					line = line.replace(&latex_cmd, "");

			   		let input_file_name = folder.to_owned() + &filename + &String::from(".tex");
					match File::open(&input_file_name) {
						Ok(f) => {
							_clean_and_copy_file(dest_file, &f, &folder);
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

		writeln!(dest_file, "{}", line).unwrap();
	}
}

fn _clean_line(line: &String) -> String {

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

fn _contain_input_file(line: &String) -> Option<(String, String)> {

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

fn assembles_filename(name :&String, folder: &String) -> String {
	return folder.clone() + &name.clone()
}

pub fn get_tmp_filename(main_tex_filename: &String, folder: &String) -> String {
	format!("{}{}{}{}",
		folder, "rumbaa_", main_tex_filename, ".tex")
}

fn delete_file_if_exist(filename: &String) {
	match remove_file(&filename) {
		Ok(()) => return,
		Err(_e) => return,
	};
}








/* ---------------------------------------------
		
					Dead code

------------------------------------------------ */

pub fn old_wrap_and_preprocess(main_tex_filename: &String, folder: &String) -> Result<File, Error> {

	// 1. Create String
	let mut tex_content: String;

	// 2. Copy everything into tmp file
	tex_content = match read_and_remove_comments(&assembles_filename(main_tex_filename, folder)) {
		Some(s) => s,
		None => {
			// Check if main tex file exists
			// If not, panic is allowed (for now)
			panic!("Unable to find {}", String::clone(folder) + main_tex_filename)
		}
	};
	
	// 3. Recursively include files
	loop {
		let re = Regex::new(r"(\\input\{)(.*?)(\})").unwrap();

	    // Modify content (reaplace input)
		let caps  = match re.captures(&tex_content) {
			Some(cap) => cap,
			None => break,
		};

   		let input_file_name = folder.to_owned() + &caps[2] + &String::from(".tex");
   		let text_file = match read_and_remove_comments(&input_file_name) {
   			Some(s) => s,
   			None => {
  				// File does not exist - replace the command by an empty string
   				println!("Warning: Unable to find {}", input_file_name);
   				String::from("")
   			}
   		};

	   	tex_content = tex_content.replace(&caps[0], &text_file);
	}

	// Removing \n
	let re = Regex::new(r"\n").unwrap();
	tex_content = re.replace_all(&tex_content, "").into_owned();


	// Copy everything in output file
	let tmp_file_name = get_tmp_filename(main_tex_filename, folder);
	delete_file_if_exist(&tmp_file_name);

	let mut main_file = OpenOptions::new()
		.create(true)
        .read(true)
        .write(true)
        .append(false)
        .open(tmp_file_name)
        ?;

	main_file.seek(SeekFrom::Start(0)).unwrap();
	writeln!(
		main_file,
		"{}", tex_content
		).unwrap();
	main_file.seek(SeekFrom::Start(0)).unwrap();

	return Ok(main_file)
}

fn read_and_remove_comments(filename: &String) -> Option<String> {
	// File to process
	let file_to_read = match File::open(filename) {
		Ok(f) => f,
		Err(_e) => {
			// File not found
			println!("Warning: {} not found",filename);
			return None
		}
	};
	let buf_reader = BufReader::new(file_to_read);

	// 1. Start reading line per line
	let mut out = String::from("");
	let mut in_long_comment = false;
	for (_num, line) in buf_reader.lines().enumerate() {
		let l = line.unwrap();
		let mut add: String;

		// 1.1 Check comments
		if l.contains("%!TEX") == true {
			// Keep line if it contains latexmk command
			add = l.clone();
		}
		else if l.contains("%") == true {
			// Contains a true comment. Try to remove it
			let split = l.split("%");
			let vec: Vec<&str> = split.collect();

			add = vec[0].to_string();
		}
		else {
			add = l.clone();
		}

		// 1.2 Check if in begin comments
		if in_long_comment == true {
			// If in long comment, check if comment ends
			if add.contains("\\end{comment}") == true {
				let split = add.split("\\end{comment}");
				let vec: Vec<&str> = split.collect();
				add = vec[1].to_string();
				in_long_comment = false;
			}
			else {
				continue;
			}
		}
		else {
			// check if long comment begins
			if add.contains("\\begin{comment}") == true {
				let split = add.split("\\begin{comment}");
				let vec: Vec<&str> = split.collect();
				add = vec[0].to_string();

				in_long_comment = true;
			}			
		}
		
		out += &add;
	}

	// 3. Output
	Some(out)
}