extern crate regex;

use std::fs;

use regex::Regex;
use crate::texstruct::Document;
use crate::texstruct::Definition;
use crate::texstruct::Theorem;

pub fn parse_tex(filename: String) -> Document {
	let contents = fs::read_to_string(String::clone(&filename))
        .expect("Something went wrong reading the file");

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

	tex_doc
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
