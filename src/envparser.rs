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

	#[derive(Clone, PartialEq)]
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
											//println!("pas de chance");
										}
									}
									self.stack_env_filtered.push(tex_env);
								}
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
			// 1. if is theorem, definition etc...
			if self.tex_struct_collection.contains_key(env_name) {
				let tex_type = self.tex_struct_collection.get(env_name).unwrap().clone();
				let math_struct = Theorem::new(String::from("NOLABEL"), tex_type);

				self.stack_theorem.push(math_struct);
				self.stack_env.push(self.current_env.clone());
				self.current_env = EnvEnumState::Theorem;
				self.stack_env_filtered.push(self.current_env.clone());
			}

			else if self.equation_env_collection.contains(&env_name) {
				self.stack_env.push(self.current_env.clone());
				self.current_env = EnvEnumState::Equation;
			}

			else if env_name == "proof" {
				let proof = Proof::new("NOTH".to_string());
				self.stack_proof.push(proof);

				self.stack_env.push(self.current_env.clone());
				self.current_env = EnvEnumState::Proof;
				self.stack_env_filtered.push(self.current_env.clone());
			}

			else {
				// We don't care
				self.stack_env.push(self.current_env.clone());
				self.current_env = EnvEnumState::Other;
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

				EnvEnumState::Proof => {
					// Do nothing
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
								println!("pas de chance");
							}
						}
						self.stack_env_filtered.push(tex_env);
					}
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
						self.doc.set_proof(&label, proof);
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
					println!("This should not happen...");
				}
			}

			self.current_env = self.stack_env.pop().unwrap();
		}
	}
}