use std::collections::HashMap;

use crate::texstruct::{TexStructure,Proof};

pub struct Document {
	pub filename: String,
	pub list_tex_struct: HashMap<String, TexStructure>,
}

impl Document {

	pub fn new (strfilename: String) -> Self {
		Self {
			filename: strfilename,
			list_tex_struct: HashMap::new(),
		}
	}
	
	pub fn push(&mut self, key: String, tex: TexStructure) -> &mut Self {
		self.list_tex_struct.insert(key, tex);
		
		// return self
		self
	}

	pub fn print(&self) -> String {
		let mut output = String::from("Printing tex structures\n");

		for (_keylabel, tex_struct) in &self.list_tex_struct {
			output = output 
				+ &tex_struct.print()
				+ "\n";
		}

		output
	}

	pub fn keys(&self) -> std::collections::hash_map::Keys<'_, String, TexStructure> {
		self.list_tex_struct.keys()
	}

	pub fn key_exist(&self, key: &String) -> bool {
		self.list_tex_struct.contains_key(key)
	}

	pub fn get(&self, key: String) -> &TexStructure {
		&self.list_tex_struct[&key]
	}

	pub fn get_name_from_key(&self, key: &String) -> &String {
		let texstruct = self.list_tex_struct.get(key).unwrap();

		texstruct.get_name()
	}

	pub fn get_group_from_key(&self, key: &String) -> i32 {
		let texstruct = self.list_tex_struct.get(key).unwrap();

		texstruct.get_group()
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