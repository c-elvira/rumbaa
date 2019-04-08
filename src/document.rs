use std::collections::HashMap;

use crate::texstruct::tex_logic::{Theorem,Proof};

pub struct Document {
	pub filename: String,
	pub list_tex_struct: HashMap<String, Theorem>,
}

impl Document {

	pub fn new (strfilename: String) -> Self {
		Self {
			filename: strfilename,
			list_tex_struct: HashMap::new(),
		}
	}
	
	pub fn push(&mut self, key: String, tex: Theorem) -> &mut Self {
		self.list_tex_struct.insert(key, tex);
		
		// return self
		self
	}

	#[allow(dead_code)]
	pub fn print(&self) -> String {
		let mut output = String::from("Printing tex structures\n");

		for (_keylabel, tex_struct) in &self.list_tex_struct {
			output = output 
				+ &tex_struct.print()
				+ "\n";
		}

		output
	}

	pub fn keys(&self) -> std::collections::hash_map::Keys<'_, String, Theorem> {
		self.list_tex_struct.keys()
	}

	pub fn contains_key(&self, key: &String) -> bool {
		self.list_tex_struct.contains_key(key)
	}

	/**
	 * @brief [brief description]
	 * @details [long description]
	 * 
	 * @param f [description]
	 * @param y [description]
	 * 
	 * @return the label of the struct that contain the label
	 * else None
	 */
	pub fn structs_contain_label(&self, key: &String) -> Option<String> {
		for (keylabel, tex_struct) in &self.list_tex_struct {
			if tex_struct.contains_equation(&key) || *keylabel == *key {
				return Some(keylabel.clone())
			}
		}

		return None
	}

	pub fn get(&self, key: &String) -> &Theorem {
		&self.list_tex_struct[key]
	}

	pub fn get_name_from_key(&self, key: &String) -> Option<&String> {
		match self.list_tex_struct.get(key) {
			Some(texstruct) => {
				return Some(texstruct.get_name())
			}
			None => {
				return None
			}
		}
	}

	pub fn get_group_from_key(&self, key: &String) -> Option<i32> {
		match self.list_tex_struct.get(key) {
			Some(texstruct) => {
				return Some(texstruct.get_group())
			}
			None => {
				return None
			}
		}
	}

	/**
	 * @brief [brief description]
	 * @details return None if key does not exist
	 * 
	 * @param f [description]
	 * @return [description]
	 */
	pub fn get_vec_dependences(&self, key: &String) -> Option<Vec<String>> {
		if !self.contains_key(key) {
			return None
		}

		// 1. create output
		let mut out: Vec<String>;
		out = Vec::new();

		// 2. get structure and proof
		let mystruct = self.get(key);
		let proof = match mystruct.get_proof() {
			Some(expr) => expr,
			None => return Some(out),
		 };

		// 3. proofs exists; iterate over links
		'loop_target: for j in 0..proof.get_nblinks() {
			match self.structs_contain_label(proof.get_link(j)) {
				Some(label) => {
					// Unwrap is safe here
					//let name_dep = self.get_name_from_key(&label).unwrap();
					out.push(label.clone());
				}
				None => continue 'loop_target,
			}
		}

		Some(out)
	}

	pub fn set_proof(&mut self, structlabel: &String, proof: Proof) {
		let texstruct = self.list_tex_struct.get_mut(structlabel).unwrap();

		texstruct.set_proof(proof);
	}

	pub fn set_label_number(&mut self, structlabel: &String, int_label: i32) {
		let texstruct = self.list_tex_struct.get_mut(structlabel).unwrap();

		texstruct.set_ilabel(int_label);
	}

	pub fn set_page(&mut self, structlabel: &String,  page: i32) {
		let texstruct = self.list_tex_struct.get_mut(structlabel).unwrap();

		texstruct.set_page(page);
	}

	pub fn set_name(&mut self, structlabel: &String, name: String) {
		let texstruct = self.list_tex_struct.get_mut(structlabel).unwrap();

		texstruct.set_name(name);
	}
}