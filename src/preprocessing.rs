extern crate tempfile;
extern crate regex;

use std::fs::{File,OpenOptions,remove_file};
use std::io::{Write,BufReader,Seek,Error,SeekFrom};
use std::io::prelude::*;
use regex::Regex;

pub fn wrap_and_preprocess(main_tex_filename: &String, folder: &String) -> Result<File, Error> {

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
	let tmp_file_name = format!("{}{}", folder, "rtex_tmp.tex");
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

fn assembles_filename(name :&String, folder: &String) -> String {
	return folder.clone() + &name.clone()
}

fn delete_file_if_exist(filename: &String) {
	match remove_file(&filename) {
		Ok(()) => return,
		Err(_e) => return,
	};
}
