extern crate tempfile;
extern crate regex;

use std::fs::{File,OpenOptions,read_to_string};
use std::io::{Write, Read, copy,Seek,Error,SeekFrom};
use regex::Regex;

pub fn build_tmp_file(texmain: &String, folder: &String) -> Result<File, Error> {

	let mut texfile = match File::open(String::clone(folder) + texmain) {
		Ok(file) => file,
		_ => panic!("Unable to find {}", String::clone(folder) + texmain),
	};

	{
		let mut tmp_file: File = File::create("tmp.tex")?;
		//let mut tmp_file = tempfile::tempfile_in(".")?;
		copy(&mut texfile, &mut tmp_file)?;
	}
	
	let mut tmp_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("tmp.tex")
        ?;
    
	_wrap_all_sources(&mut tmp_file, folder)?;

	Ok(tmp_file)
}

fn _wrap_all_sources(main_file: &mut File, folder: &String) -> std::io::Result<()> {

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