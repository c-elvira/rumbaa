extern crate regex;

use std::collections::HashMap;

use std::fs::File;
use std::io::{BufReader};
use std::io::prelude::*;

use crate::texstruct::{TexStructure,EnumTexType,clone_tex_type,Proof};
use crate::document::{Document};

use regex::Regex;


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
pub fn parse_tex(main_clean_file: &File, main_filename: &String, _folder: &String) -> std::io::Result<(Document)> {

	// Process it
	let mut contents = String::new();
	let mut buf_reader = BufReader::new(main_clean_file);
	buf_reader.read_to_string(&mut contents).unwrap();

		// Creating document
	let mut tex_doc = Document::new(main_filename.to_string());

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


fn build_tex_struct_collection(text: &String) -> HashMap<String, EnumTexType> {
	
	// Initial Hashmap
    let mut tex_structure_collection = hashmap![
    	"definition".to_string()  => EnumTexType::Definition,
    	"theorem".to_string() 	  => EnumTexType::Theorem,
    	"proposition".to_string() => EnumTexType::Proposition,
    	"lemma".to_string()		  => EnumTexType::Lemma,
    	"corollary".to_string()   => EnumTexType::Corollary
		];

	// Looking for new structure
	// \newtheorem{name}{Printed output}
	let str_regex = r"\\newtheorem\{(.*?)\}\{(.*?)\}";
	let regex_def = Regex::new(&str_regex).unwrap();
	for cap in regex_def.captures_iter(&text) {
		let new_keyword = cap[1].to_string();

		if !tex_structure_collection.contains_key(&new_keyword) {
			tex_structure_collection.insert(new_keyword, EnumTexType::Custom);
		}
	}

	return tex_structure_collection
}


fn find_structs(text: &String, keyword: &String, tex_type: &EnumTexType, doc: &mut Document) {
	
	let str_regex = format!(r"(\\begin\{{{}\}})(.*?)(\\end\{{{}\}})", keyword, keyword);
	let regex_def = Regex::new(&str_regex).unwrap();
	for cap in regex_def.captures_iter(&text) {
		match find_label(&cap[2].to_string()) {
			Some(strlabel) => {
				//let cleantext = remove_label(cap[2].to_string(), &strlabel);

				// Create structure
				let mut math_struct = TexStructure::new(String::clone(&strlabel), clone_tex_type(tex_type));

				// Add equation labels
				seeks_equations(&cap[2].to_string(), &mut math_struct);

				// Add structure to document
				doc.push(strlabel, math_struct);
			}
			None => {
				continue
			}
		}
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
		if doc.contains_key(&associated_th[1].to_string()) == false {
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


fn find_label(text: &String) -> Option<String> {

	let re_label = Regex::new(r"(\\label\{)(.*?)(\})").unwrap();
	match re_label.captures(&text) {
    	Some(caps) => {
        	let cap = caps.get(2).unwrap().as_str();
        	return Some(cap.to_string())
    	}
    	None => {
			return None
    	}
	}

	//let cap = re_label.captures(&text).unwrap();
	//cap[2].to_string()
}


fn seeks_equations(text: &String, math_struct: &mut TexStructure) {

	let supported_eq_env = vec![
		"equation".to_string(), 
		"align".to_string(),
		"multline".to_string(),
		"gather".to_string(),
		"eqnarray".to_string()
		];

	for elem in supported_eq_env.iter() {
		//1. Create regex
		let str_regex = format!(r"(\\begin\{{{}\}})(.*?)(\\end\{{{}\}})", elem, elem);
		let regex_eq = Regex::new(&str_regex).unwrap();

		// 2. Iterature over \begin{eq} ... \end{eq}
		'eq_loop: for cap in regex_eq.captures_iter(&text) {
			// While label are found, continue
			let mut _eq_text = cap[2].to_string().clone();

			'label_loop: loop {
				// 2. find all label 
				match find_label(&_eq_text) {
					Some(strlabel) => {
						// 2.1. remove found label
						let full_label = "\\label{".to_owned() + &strlabel + "}";
						_eq_text = _eq_text.replace(&full_label[..], "");

						// 2.2. add label to structure
						math_struct.add_equation(strlabel);
					}
					None => {
						// If not label is found, quit loop
						break 'label_loop;
					}
				}
			}
		}
	}
}
