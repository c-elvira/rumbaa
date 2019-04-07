extern crate log;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

pub mod texparser {
	use std::collections::HashMap;

	use crate::texstruct::tex_logic::{EnumMacroType,TexMacro};
	use crate::document::{Document};
	use crate::texstruct::tex_logic::{Theorem,EnumTheoremType,Proof};

	#[derive(Clone, Debug, PartialEq)]
	enum EnvEnumState {
		None,
		Theorem, // definition, theorem, custom
		Proof,
		Equation,
		Other,
	}

	pub struct EnvParser<'a> {
		current_env: EnvEnumState,
		stack_env: Vec<EnvEnumState>,
		stack_env_filtered: Vec<EnvEnumState>,
		stack_theorem: Vec<Theorem>,
		stack_proof: Vec<Proof>,
		tex_struct_collection: HashMap<String, EnumTheoremType>,
		equation_env_collection: Vec<String>,

		no_label_count: i32,
		doc: &'a mut Document,
	}

	impl<'a> EnvParser<'a> {
		pub fn new(doc_input: &'a mut Document) -> Self {
	        EnvParser {
	        	current_env: EnvEnumState::None,
	            stack_env: Vec::new(),
	            stack_env_filtered: Vec::new(),
	            stack_theorem: Vec::new(),
	            stack_proof: Vec::new(),

	            no_label_count: 0,
		        doc: doc_input,
	    
	            tex_struct_collection: hashmap![
    				"definition".to_string()  => EnumTheoremType::Definition,
    				"theorem".to_string() 	  => EnumTheoremType::Theorem,
    				"proposition".to_string() => EnumTheoremType::Proposition,
    				"lemma".to_string()		  => EnumTheoremType::Lemma,
    				"corollary".to_string()   => EnumTheoremType::Corollary
				],

				equation_env_collection: vec![
					"equation".to_string(),
					"align".to_string(),
					"multlines".to_string(),
				]
	        }
	    }


		pub fn process_macro(&mut self, tex_macro: &TexMacro) {

			match tex_macro.get_macro_type() {

				EnumMacroType::Tex => {

					info!("Process Tex Macro: {} - {:?}", tex_macro.get_name(), tex_macro.get_args());

					match tex_macro.get_name().as_ref() {
						"newtheorem" => {
							let keyword = tex_macro.get_arg(0);

							if !self.tex_struct_collection.contains_key(&keyword) {
								self.tex_struct_collection.insert(keyword, EnumTheoremType::Custom);
							}
						}

						"begin" => {
							// process environment
							let env_name = tex_macro.get_arg(0);
							self.open_env(&env_name);
						}

						"end" => {
							// close environment
							self.close_env();
						}

						"label" => {
							let label = tex_macro.get_arg(0);
							self.add_label_to_env(label);
						}

						_ => {
							if tex_macro.get_name().contains("ref") {
								self.manage_reference(&tex_macro);
							}
						}
					}
				}

				EnumMacroType::LatexMk => {
					info!("Process LatexMk Macro: {} - {:?}", tex_macro.get_name(), tex_macro.get_args());

					match tex_macro.get_name().as_ref() {

						"proof" => {
							if self.current_env == EnvEnumState::Proof {
								let label = &tex_macro.get_arg(0);

								info!("Tex parser has found proof of {}", label);
								let mut proof = self.stack_proof.pop().unwrap();
								proof.set_struct_label(label);
								self.stack_proof.push(proof);
							}
						}

						_ => {
							// Do nothing
						}
					}
				}
			}
		}

		fn open_env(&mut self, env_name: &String) {
			self.stack_env.push(self.current_env.clone());

			if self.tex_struct_collection.contains_key(env_name) {
				// 1. if is theorem, definition etc...
				self.current_env = EnvEnumState::Theorem;

				let tex_type = self.tex_struct_collection.get(env_name).unwrap().clone();

				let math_struct = Theorem::new(String::from("NOLABEL"), tex_type);
				self.stack_theorem.push(math_struct);
			}

			else if env_name == "proof" {
				self.current_env = EnvEnumState::Proof;

				let proof = Proof::new("NOTH".to_string());
				self.stack_proof.push(proof);
			}

			else if self.equation_env_collection.contains(&env_name) {
				self.current_env = EnvEnumState::Equation;
			}

			else {
				self.current_env = EnvEnumState::Other;
			}

			match self.current_env {
				EnvEnumState::Theorem | EnvEnumState::Proof => {
					self.stack_env_filtered.push(self.current_env.clone());
				}
				_ => ()
			}
		}

		fn add_label_to_env(&mut self, label: String) {
			match self.current_env {
				EnvEnumState::Theorem => {
					// Add label to theorem
					let mut math_struct = self.stack_theorem.pop().unwrap();
					math_struct.set_label(&label);
					self.stack_theorem.push(math_struct);
				}

				EnvEnumState::Equation => {
					// Add label to Theorem container if it exists
					if self.stack_env_filtered.len() > 0 {
						let tex_env = self.stack_env_filtered.pop().unwrap();
						match tex_env {
							EnvEnumState::Theorem => {
								let mut math_struct = self.stack_theorem.pop().unwrap();
								info!("add {} to {}", label, math_struct.clone_label());
								math_struct.add_equation(label);
								self.stack_theorem.push(math_struct);
							}

							_ => {
								// Do noting
							}
						}
						self.stack_env_filtered.push(tex_env);
					}
				}

				EnvEnumState::Proof => {
					// Do nothing
				}

				EnvEnumState::Other => {
					// Do nothing
				}

				EnvEnumState::None => {
					// Do nothing
				}
			}
		}

		fn close_env(&mut self) {
			match self.current_env {

				EnvEnumState::Theorem => {
					let mut math_struct = self.stack_theorem.pop().unwrap();
					let mut label = math_struct.clone_label();
					if label == "NOLABEL" {
						self.no_label_count += 1;
						label = format!("{}-{}", label, self.no_label_count);
						math_struct.set_label(&label);
					}
					self.doc.push(label, math_struct);
					self.stack_env_filtered.pop().unwrap();
				}

				EnvEnumState::Proof => {
					let proof = self.stack_proof.pop().unwrap();
					let label = proof.get_struct_label();

					if label != "NOTH" {
						if self.doc.contains_key(&label) {
							self.doc.set_proof(&label, proof);
						}
						else {
							warn!("Theorem {} not found", label)
						}
					}

					self.stack_env_filtered.pop().unwrap();
				}

				EnvEnumState::Equation => {
					// Nothing to do also
				}

				EnvEnumState::Other => {
					// Do nothing
				}

				EnvEnumState::None => {
					// Error
					println!("Closing None env: this should not happen...");
				}
			}

			self.current_env = self.stack_env.pop().unwrap();
		}

		fn manage_reference(&mut self, tex_macro: &TexMacro) {
			// Add reference to proof container if it exists
			if self.stack_env_filtered.len() > 0 {
				let tex_env = self.stack_env_filtered.pop().unwrap();

				match tex_env {
				
					EnvEnumState::Proof => {
						let label = tex_macro.get_arg(0);
						let mut proof = self.stack_proof.pop().unwrap();
			
						//info!("add {} to {}", label, math_struct.clone_label());
						proof.add_link(label);
						self.stack_proof.push(proof);
					}

					_ => {
						// Do noting
					}
				}
			
				self.stack_env_filtered.push(tex_env);
			}
		}
	}

	/* -------------------------------

				Tests

	------------------------------- */


	#[cfg(test)]
	mod tests {

		use crate::document::{Document};
		use crate::texstruct::tex_logic::{EnumMacroType,TexMacro};
		use crate::envparser::texparser::{EnvParser};

		fn tex_macro_builder(name: String, arg1: String) -> TexMacro {
			let mut macro_out = TexMacro::new(EnumMacroType::Tex);
			macro_out.set_name(&name);
			macro_out.add_arg(&arg1);

			macro_out		
		}

		fn mk_macro_builder(name: String, arg1: String) -> TexMacro {
			let mut macro_out = TexMacro::new(EnumMacroType::LatexMk);
			macro_out.set_name(&name);
			macro_out.add_arg(&arg1);

			macro_out		
		}

		#[test]
		fn open_and_close_env() {
			let mut doc = Document::new("filename".to_string());
			let mut env_parser = EnvParser::new(&mut doc);

			let open_macro = tex_macro_builder(String::from("begin"), String::from("theorem"));
			let label_macro = tex_macro_builder(String::from("label"), String::from("th:name"));
			let close_macro = tex_macro_builder(String::from("end"), String::from("theorem"));

			env_parser.process_macro(&open_macro);
			env_parser.process_macro(&label_macro);
			env_parser.process_macro(&close_macro);

			assert!(doc.contains_key(&String::from("th:name")))
		}

		#[test]
		fn equation_in_def() {
			let mut doc = Document::new("filename".to_string());
			let mut env_parser = EnvParser::new(&mut doc);

			let open_def = tex_macro_builder(String::from("begin"), String::from("definition"));
			let open_eq = tex_macro_builder(String::from("begin"), String::from("equation"));
			let label_eq = tex_macro_builder(String::from("label"), String::from("eq:name"));
			let close_eq = tex_macro_builder(String::from("end"), String::from("equation"));
			let close_def = tex_macro_builder(String::from("end"), String::from("definition"));

			let open_th = tex_macro_builder(String::from("begin"), String::from("theorem"));
			let label_th = tex_macro_builder(String::from("label"), String::from("th:name"));
			let close_th = tex_macro_builder(String::from("end"), String::from("theorem"));

			let open_proof = tex_macro_builder(String::from("begin"), String::from("proof"));
			let proof_of = mk_macro_builder(String::from("proof"), String::from("th:name"));
			let ref_in_proof = tex_macro_builder(String::from("ref"), String::from("eq:name"));
			let end_proof = tex_macro_builder(String::from("end"), String::from("proof"));

			env_parser.process_macro(&open_def);
			env_parser.process_macro(&open_eq);
			env_parser.process_macro(&label_eq);
			env_parser.process_macro(&close_eq);
			env_parser.process_macro(&close_def);

			env_parser.process_macro(&open_th);
			env_parser.process_macro(&label_th);
			env_parser.process_macro(&close_th);

			env_parser.process_macro(&open_proof);
			env_parser.process_macro(&proof_of);
			env_parser.process_macro(&ref_in_proof);
			env_parser.process_macro(&end_proof);

			assert!(doc.contains_key(&String::from("th:name")));

			let dep = doc.get_vec_dependences(&"th:name".to_string());
			assert!(!dep.is_none());
			let vec_dep = dep.unwrap();
			assert!(vec_dep.len() == 1);
		}
	}
}