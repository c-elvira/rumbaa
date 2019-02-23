extern crate regex;

use std::io::{BufReader};
use std::io::prelude::*;

use crate::texstruct::{Definition,Theorem,Lemma,Proposition,Proof,Corollary};
use crate::document::{Document};

use regex::Regex;
use crate::textmpfile::build_tmp_file;

/**
 * @brief [brief description]
 * @details Implement the state machine
 * 
 * @param e [description]
 * @return [description]
 */
pub fn parse_tex(filename: &String, folder: &String) -> std::io::Result<(Document)> {


	// get clean tex
	let tmp_file = build_tmp_file(&filename, &folder)?;

	// Process it
	let mut contents = String::new();
	let mut buf_reader = BufReader::new(tmp_file);
	buf_reader.read_to_string(&mut contents).unwrap();
	// tmp_file.read_to_string(&mut contents)

		// Creating document
	let mut tex_doc = Document::new(filename.to_string());

		// Removing \n
	let re = Regex::new(r"\n").unwrap();
	let contents = re.replace_all(&contents, "").into_owned();

	// 1. Looking for:
	// 	- definitions
	//	- theorems
	process_definition(&contents, &mut tex_doc);
	process_theorem(&contents, &mut tex_doc);
	process_lemma(&contents, &mut tex_doc);
	process_proposition(&contents, &mut tex_doc);
	process_corollary(&contents, &mut tex_doc);

	// 2. Finaly process proofs
	process_proofs(&contents, &mut tex_doc);

	Ok(tex_doc)
}

fn process_definition(text: &String, doc: &mut Document) {
	
	let regex_def = Regex::new(r"(\\begin\{definition\})(.*?)(\\end\{definition\})").unwrap();
	for cap in regex_def.captures_iter(&text) {	
		let strlabel = find_label(&cap[2].to_string());
		//let cleantext = remove_label(cap[2].to_string(), &strlabel);

		let def = Definition::new(String::clone(&strlabel));
		doc.push(String::clone(&strlabel), def);
	}
}


fn process_theorem(text: &String, doc: &mut Document) {
	
	let regex_theorem = Regex::new(r"(\\begin\{theorem\})(.*?)(\\end\{theorem\})").unwrap();
	for cap in regex_theorem.captures_iter(&text) {	
		let strlabel = find_label(&cap[2].to_string());
		//let cleantext = remove_label(cap[2].to_string(), &strlabel);

		let th = Theorem::new(String::clone(&strlabel));
		doc.push(strlabel, th);
	}
}


fn process_lemma(text: &String, doc: &mut Document) {
	
	let regex_lemma = Regex::new(r"(\\begin\{lemma\})(.*?)(\\end\{lemma\})").unwrap();
	for cap in regex_lemma.captures_iter(&text) {	
		let strlabel = find_label(&cap[2].to_string());

		let lemma = Lemma::new(String::clone(&strlabel));
		doc.push(strlabel, lemma);
	}
}


fn process_proposition(text: &String, doc: &mut Document) {
	
	let regex_prop = Regex::new(r"(\\begin\{proposition\})(.*?)(\\end\{proposition\})").unwrap();
	for cap in regex_prop.captures_iter(&text) {	
		let strlabel = find_label(&cap[2].to_string());

		let prop = Proposition::new(String::clone(&strlabel));
		doc.push(strlabel, prop);
	}
}


fn process_corollary(text: &String, doc: &mut Document) {
	
	let regex_prop = Regex::new(r"(\\begin\{corollary\})(.*?)(\\end\{corollary\})").unwrap();
	for cap in regex_prop.captures_iter(&text) {	
		let strlabel = find_label(&cap[2].to_string());

		let corr = Corollary::new(String::clone(&strlabel));
		doc.push(strlabel, corr);
	}
}


fn process_proofs(text: &String, doc: &mut Document) {

	let regex_proof = Regex::new(r"(\\begin\{proof\})(.*?)(\\end\{proof\})").unwrap();

	'loop_proof: for cap in regex_proof.captures_iter(&text) {
		let rtex_proof_patern = r"!TEX proof = \{(.*?)\}";
		let regex_latexmk = Regex::new(rtex_proof_patern).unwrap();

		// 1. Get associated Theorem (or lemma) 
		let content = String::clone(&cap[2].to_string());
		let associated_th = match regex_latexmk.captures(&content) {
			Some(cap_label) => cap_label,
			None => continue,
		};
		if doc.key_exist(&associated_th[1].to_string()) == false {
			continue 'loop_proof;
		}

		// 2. Create proof
		let mut proof = Proof::new(String::clone(&associated_th[1].to_string()));

		// 3. look for reference
		let ref_patern = r"ref\{(.*?)\}";
		let regex_ref = Regex::new(ref_patern).unwrap();
		for cap_ref in regex_ref.captures_iter(&content) {
			proof.add_link(cap_ref[1].to_string());
		}

		// 4. Transfert ownership to doc
		doc.set_proof(&associated_th[1].to_string(), proof);
	}
}

fn find_label(text: &String) -> String {
	
	let re_label = Regex::new(r"(\\label\{)(.*?)(\})").unwrap();
	let cap = re_label.captures(&text).unwrap();

	cap[2].to_string()
}
