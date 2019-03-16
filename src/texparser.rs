extern crate regex;

use std::collections::HashMap;

use std::io::{BufReader};
use std::io::prelude::*;

use crate::texstruct::{TexStructure,EnumTexType,clone_tex_type,Proof};
use crate::document::{Document};

use regex::Regex;
use crate::textmpfile::build_tmp_file;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

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

	// HashMap<&str, EnumTexType> =
    let tex_structure_collection = build_tex_struct_collection(&contents);

	for (keyword, tex_type) in tex_structure_collection {
		find_structs(&contents, &keyword, &tex_type, &mut tex_doc);
	}

	// 2. Finaly process proofs
	process_proofs(&contents, &mut tex_doc);

	Ok(tex_doc)
}


fn build_tex_struct_collection(_text: &String) -> HashMap<String, EnumTexType> {
	
	// HashMap<&str, EnumTexType> =
    let tex_structure_collection = hashmap![
    	"definition".to_string()  => EnumTexType::Definition,
    	"theorem".to_string() 	  => EnumTexType::Theorem,
    	"proposition".to_string() => EnumTexType::Proposition,
    	"lemma".to_string()		  => EnumTexType::Lemma,
    	"corollary".to_string()   => EnumTexType::Corollary,
       	"custom".to_string()   => EnumTexType::Other
		];

	return tex_structure_collection
}


fn find_structs(text: &String, keyword: &String, tex_type: &EnumTexType, doc: &mut Document) {
	
	let str_regex = format!(r"(\\begin\{{{}\}})(.*?)(\\end\{{{}\}})", keyword, keyword);
	// "(\\begin\{definition\})(.*?)(\\end\{definition\})"
	let regex_def = Regex::new(&str_regex).unwrap();
	for cap in regex_def.captures_iter(&text) {
		//println!("{:?}", cap);
		let strlabel = find_label(&cap[2].to_string());
		//let cleantext = remove_label(cap[2].to_string(), &strlabel);

		let def = TexStructure::new(String::clone(&strlabel), clone_tex_type(tex_type));
		doc.push(strlabel, def);
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
