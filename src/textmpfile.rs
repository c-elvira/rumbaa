extern crate tempfile;
extern crate regex;

use std::fs::{File,OpenOptions,read_to_string,remove_file};
use std::io::{Write,BufReader,Read, copy,Seek,Error,SeekFrom};
use std::io::prelude::*;
use regex::Regex;

pub fn build_tmp_file(texmain: &String, folder: &String) -> Result<File, Error> {

	let mut texfile = match File::open(String::clone(folder) + texmain) {
		Ok(file) => file,
		_ => panic!("Unable to find {}", String::clone(folder) + texmain),
	};

	{
		let mut tmp_file: File = File::create(format!("{}{}", folder, "rtex_tmp.tex"))?;
		copy(&mut texfile, &mut tmp_file)?;
	}
	
	let tmp_file_name_1 = format!("{}{}", folder, "rtex_tmp.tex");
	wrap_all_sources(&tmp_file_name_1, folder)?;

	let tmp_file_name_2 = format!("{}{}", folder, "rtex_tmp2.tex");
	match remove_comments(&tmp_file_name_2, &tmp_file_name_1) {
		Ok(()) => {
			remove_file(tmp_file_name_1).unwrap();
			return Ok(File::open(&tmp_file_name_2)?)
		},
		Err(_)  => {
			println!("Something wrong happened while removing comments from tex file");
			println!("Continue anayway");

			return Ok(File::open(&tmp_file_name_1)?)
		},
	};
}

fn wrap_all_sources(main_file_name: &String, folder: &String) -> std::io::Result<()> {

	let mut main_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(main_file_name)
        ?;

	let mut contents = String::new();
	main_file.read_to_string(&mut contents)?;

	loop {
		let re = Regex::new(r"(\\input\{)(.*?)(\})").unwrap();

	    // Modify content (reaplace input)
		let caps  = match re.captures(&contents) {
			Some(cap) => cap,
			None => break,
		};

   		let input_file_name = folder.to_owned() + &caps[2] + &String::from(".tex");
	   	let text_file = match read_to_string(&input_file_name) {
			Ok(file) => file,
			_ => {
				// File does not exist - replace the command by an empty string
				println!("Unable to find file {}", input_file_name);
				String::from("Unable to find file ".to_owned() + &input_file_name)
			},
		};
	   	contents = contents.replace(&caps[0], &text_file);
	}

	// Replace all
	main_file.seek(SeekFrom::Start(0)).unwrap();
	writeln!(
        main_file,
        "{}", contents
        ).unwrap();
	main_file.seek(SeekFrom::Start(0)).unwrap();

	Ok(())
}

fn remove_comments(new_tmpfile_name: &String, tmp_file1_name: &String) -> std::io::Result<()> {

	// File to process
	let tmp_file1 = File::open(tmp_file1_name)?;
	let buf_reader = BufReader::new(tmp_file1);

    let mut new_tmp_file: File = File::create(&new_tmpfile_name)?;

	for (_num, line) in buf_reader.lines().enumerate() {
    	let l = line.unwrap();

    	if l.contains("%") == false {
			writeln!(
    		    new_tmp_file,
    	    	"{}", l
	        ).unwrap();
    	}
    	else if l.contains("%!TEX") == true {
			writeln!(
    		    new_tmp_file,
    	    	"{}", l
	        ).unwrap();
    	}
    	else {
    		// Contains a true comment. Try to remove it
       		let split = l.split("%");
    		let vec: Vec<&str> = split.collect();
    
   			writeln!(
    		    new_tmp_file,
    	    	"{}", vec[0]
	        ).unwrap();
    	}
	}

	Ok(())
}
