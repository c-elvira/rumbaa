extern crate regex;

use std::io::{BufReader};
use std::io::prelude::*;

use regex::Regex;
use crate::texstruct::{Document,Definition,Theorem};
use crate::textmpfile::build_tmp_file;

pub fn parse_tex(filename: String) -> std::io::Result<(Document)> {

	// get clean tex
	let tmp_file = build_tmp_file(&filename)?;
	//tmp_file.seek(SeekFrom::Start(0)).unwrap();

	// Process it
    let mut contents = String::new();
    let mut buf_reader = BufReader::new(tmp_file);
    buf_reader.read_to_string(&mut contents).unwrap();
    // tmp_file.read_to_string(&mut contents)

        // Creating document
	let mut tex_doc = Document::new(filename);

		// Removing \n
	let re = Regex::new(r"\n").unwrap();
	let contents = re.replace_all(&contents, "");

		// 1. Looking for definition
	let re_def = Regex::new(r"(\\begin\{definition\})(.*?)(\\end\{definition\})").unwrap();
	for cap in re_def.captures_iter(&contents) {	
    	let strlabel = find_label(cap[2].to_string());
    	let text = remove_label(cap[2].to_string(), &strlabel);

    	let def = Definition::new(String::clone(&strlabel), text);
    	tex_doc.push(String::clone(&strlabel), def);
	}

		// 2. Looking for Theorem
	let re_def = Regex::new(r"(\\begin\{theorem\})(.*?)(\\end\{theorem\})").unwrap();
	for cap in re_def.captures_iter(&contents) {	
    	let strlabel = find_label(cap[2].to_string());
    	let text = remove_label(cap[2].to_string(), &strlabel);

    	let th = Theorem::new(String::clone(&strlabel), text);
    	tex_doc.push(strlabel, th);
	}

	Ok(tex_doc)
}

fn find_label(text: String) -> String {
	
	let re_label = Regex::new(r"(\\label\{)(.*?)(\})").unwrap();
	let cap = re_label.captures(&text).unwrap();

	cap[2].to_string()
}

fn remove_label(text: String, label: &String) -> String {
	let mut re = Regex::new(&label).unwrap();
	let contents = re.replace_all(&text, "");

	re = Regex::new(r"\\label\{\}").unwrap();
	let contents = re.replace_all(&contents, "");

	contents.to_string()
}
