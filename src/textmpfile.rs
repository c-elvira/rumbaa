extern crate tempfile;
extern crate regex;

use std::fs::{File,OpenOptions,read_to_string};
use std::io::{Write, Read, copy,Seek,Error,SeekFrom};
use regex::Regex;

pub fn build_tmp_file(texmain: &String) -> Result<File, Error> {

	let mut texfile = File::open(texmain)?;
	{
		let mut tmpfile: File = File::create("temp/tmp.tex")?;
		//tempfile::tempfile_in("temp/")?;
		copy(&mut texfile, &mut tmpfile)?;
	}

	//let mut tmpfile = File::open("temp/tmp.tex")?;
	let mut tmpfile = OpenOptions::new()
        .read(true)
        .write(true)
        .open("temp/tmp.tex")
        ?;
	_wrap_all_sources(&mut tmpfile)?;

	Ok(tmpfile)
}

fn _wrap_all_sources(main_file: &mut File) -> std::io::Result<()> {

	let folder = "texdata/";

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
	   	let text_file = read_to_string(&input_file_name)?;
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