extern crate log;





macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}


pub mod texparser {
	
	use std::fs::File;
	use std::io::{BufRead,BufReader};

	use std::collections::HashMap;
	//use std::io::prelude::*;

	use crate::texstruct::{TexStructure,EnumTexType,clone_tex_type,Proof};
	use crate::document::{Document};

	/**
	 * @brief [brief description]
	 * @details Implement the state machine
	 * 
	 * @param e [description]
	 * @return [description]
	 */
	pub fn parse_tex(main_clean_file: &File, main_filename: &String) -> std::io::Result<(Document)> {
			// Creating document
		let mut tex_doc = Document::new(main_filename.to_string());

			// Creating tex parser
	    let mut parser = TexParser::new(&mut tex_doc);

		// 1. Reading
	    let mut reader = BufReader::new(main_clean_file);
	    let mut buf = Vec::<u8>::new();

	    // -- starting reading loop
	    while reader.read_until(b'\n', &mut buf).expect("read_until failed") != 0 {
	        // this moves the ownership of the read data to s
	        // there is no allocation
	        let s = String::from_utf8(buf).expect("from_utf8 failed");
   	        for c in s.chars() {
   		     	parser.add_char(c);
   		    }

	        // this returns the ownership of the read data to buf
	        // there is no allocation
	        buf = s.into_bytes();
	        buf.clear();
	    }

		Ok(tex_doc)
	}

	#[derive(Debug)]
	struct TexCmd {
		name: String,
		args: Vec<String>,
		option_args: Vec<String>,
	}

	impl TexCmd {
		pub fn new (cmd_name: &String) -> Self {
			Self {
				name: cmd_name.clone(),
				args: Vec::new(),
				option_args: Vec::new(),
			}
		}

		fn set_name(&mut self, cmd_name: &String) {
			self.name = cmd_name.clone();
		}

		fn add_arg(&mut self, arg: &String) {
			self.args.push(arg.clone());
		}

		fn add_opt_arg(&mut self, opt_arg: &String) {
			self.option_args.push(opt_arg.clone());
		}
	}

	#[derive(Debug, Clone)]
	enum TexParserState {
		Empty,
		//InMacroName,
		InMacro,
		InMacroOptionalArg,
		InMacroArg,
		InComment,
	}

	struct TexParser<'a> {
		env_parser: EnvParser<'a>,
		current_state: TexParserState,
		stack_state: Vec<TexParserState>,
		stack_macro: Vec<TexCmd>,
		bufcmd: String,
		buf_comment: String,
	}

	impl<'a> TexParser<'a> {
		fn new(doc_input: &'a mut Document) -> Self {
	        TexParser {
				current_state: TexParserState::Empty,
	            stack_state: Vec::new(),
	            env_parser: EnvParser::new(doc_input),
	            bufcmd: String::from(""),
	            buf_comment: String::from(""),
	            stack_macro: Vec::new(),
	        }
	    }

	    /**
	     * @brief State macine main logic
	     */
	    fn add_char(&mut self, c: char) {
	    	match self.current_state {

	    		TexParserState::Empty => {

	    			match c {

	    				'\\' => {
		    				let texmacro = TexCmd::new(&String::from(""));
		    				self.stack_macro.push(texmacro);

	    					self.stack_state.push(self.current_state.clone());
	    					self.current_state = TexParserState::InMacro;
	    				}

	    				'%' => {
	    					self.stack_state.push(self.current_state.clone());
	    					self.current_state = TexParserState::InComment;
	    				}

	    				_ => {
	    					// Do nothing
	    				}
			    	}
	    		}

	    		TexParserState::InMacro => {

	    			if c.is_alphabetic() {
	    				// still in macro name
	    				self.bufcmd.push(c);
	    			}
	    			else if c == '\\' {
	    				// Start another macro
	    				self.set_macro_name_from_buf();
    					let tex_macro = self.stack_macro.pop().unwrap();
						self.env_parser.process_macro(tex_macro);

	    				let texmacro = TexCmd::new(&String::from(""));
	    				self.stack_macro.push(texmacro);    				
	    			}
					else if c == ' ' {
						if self.bufcmd != "" {
							self.set_macro_name_from_buf();
						}
    					let tex_macro = self.stack_macro.pop().unwrap();
						self.env_parser.process_macro(tex_macro);

	    				self.current_state = self.stack_state.pop().unwrap();
					}

					else if c == '[' {
						if self.bufcmd != "" {
							self.set_macro_name_from_buf();
						}

						self.stack_state.push(self.current_state.clone());
						self.current_state = TexParserState::InMacroOptionalArg;
					}

					else if c == '{' {
						if self.bufcmd != "" {
							self.set_macro_name_from_buf();
						}

						self.stack_state.push(self.current_state.clone());
						self.current_state = TexParserState::InMacroArg;
					}

					else if c == '%' {
    					self.stack_state.push(self.current_state.clone());
    					self.current_state = TexParserState::InComment;
					}

					else {
						if self.bufcmd != "" {
							self.set_macro_name_from_buf();
						}

    					let tex_macro = self.stack_macro.pop().unwrap();
						self.env_parser.process_macro(tex_macro);

	    				self.current_state = self.stack_state.pop().unwrap();
					}
	    		}

	    		TexParserState::InMacroOptionalArg => {
	    			match c {
	    				']' => {
							let mut tex_macro = self.stack_macro.pop().unwrap();
							tex_macro.add_opt_arg(&self.bufcmd);
							self.stack_macro.push(tex_macro);

							self.bufcmd = String::from("");
							self.current_state = self.stack_state.pop().unwrap();
	    				}

	    				'%' => {
	    					self.stack_state.push(self.current_state.clone());
    						self.current_state = TexParserState::InComment;
	    				}

	    				_ => {
	    					self.bufcmd.push(c);
	    				}
	    			}
	    		}

				TexParserState::InMacroArg => {
	    			match c {
	    				'}' => {
							let mut tex_macro = self.stack_macro.pop().unwrap();
							tex_macro.add_arg(&self.bufcmd);
							self.stack_macro.push(tex_macro);

							self.bufcmd = String::from("");
							self.current_state = self.stack_state.pop().unwrap();
	    				}

	    				'%' => {
	    					self.stack_state.push(self.current_state.clone());
    						self.current_state = TexParserState::InComment;
	    				}

	    				_ => {
	    					self.bufcmd.push(c);
	    				}
	    			}
				}

				TexParserState::InComment => {
					match c {
						'\n' => {
							// End of comment, process it
							self.env_parser.check_latexmk_macro(&self.buf_comment);

							self.buf_comment = "".to_string();
							self.current_state = self.stack_state.pop().unwrap();
						}

						_ => {
							self.buf_comment.push(c);
						}
					}
				}
	    	}
	    }

	    fn set_macro_name_from_buf(&mut self) {
			let mut tex_macro = self.stack_macro.pop().unwrap();
			tex_macro.set_name(&self.bufcmd);
			self.bufcmd = String::from("");

			self.stack_macro.push(tex_macro);
	    }
	}

	#[derive(Clone)]
	enum EnvEnumState {
		None,
		Theorem, // definition, theorem, custom
		Proof,
		Equation,
		Other,
	}


	struct EnvParser<'a> {
		current_env: EnvEnumState,
		stack_env: Vec<EnvEnumState>,
		stack_env_filtered: Vec<EnvEnumState>,
		stack_theorem: Vec<TexStructure>,
		stack_proof: Vec<Proof>,
		tex_struct_collection: HashMap<String, EnumTexType>,
		equation_env_collection: Vec<String>,

		no_label_count: i32,
		doc: &'a mut Document,
	}

	impl<'a> EnvParser<'a> {
		fn new(doc_input: &'a mut Document) -> Self {
	        EnvParser {
	        	current_env: EnvEnumState::None,
	            stack_env: Vec::new(),
	            stack_env_filtered: Vec::new(),
	            stack_theorem: Vec::new(),
	            stack_proof: Vec::new(),

	            no_label_count: 0,
		        doc: doc_input,
	    
	            tex_struct_collection: hashmap![
    				"definition".to_string()  => EnumTexType::Definition,
    				"theorem".to_string() 	  => EnumTexType::Theorem,
    				"proposition".to_string() => EnumTexType::Proposition,
    				"lemma".to_string()		  => EnumTexType::Lemma,
    				"corollary".to_string()   => EnumTexType::Corollary
				],

				equation_env_collection: vec![
					"equation".to_string(),
					"align".to_string(),
					"multlines".to_string(),
				]
	        }
	    }


		fn process_macro(&mut self, tex_macro: TexCmd) {
			info!("Process cmd: {} - {:?}", tex_macro.name, tex_macro.args);

			match tex_macro.name.as_ref() {
				"newtheorem" => {
					let keyword = tex_macro.args[0].clone();

					if !self.tex_struct_collection.contains_key(&keyword) {
						self.tex_struct_collection.insert(keyword, EnumTexType::Custom);
					}
				}

				"begin" => {
					// process environment
					let env_name = tex_macro.args[0].clone();
					self.open_env(&env_name);
				}

				"end" => {
					// close environment
					self.close_env();
				}

				"label" => {
					let label = tex_macro.args[0].clone();
					self.add_label_to_env(label);
				}

				_ => {
					if tex_macro.name.contains("ref") {
					// Add reference to proof container if it exists
					if self.stack_env_filtered.len() > 0 {
						let tex_env = self.stack_env_filtered.pop().unwrap();
						match tex_env {
							EnvEnumState::Proof => {
								let label = tex_macro.args[0].clone();
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

		fn open_env(&mut self, env_name: &String) {
			// 1. if is theorem, definition etc...
			if self.tex_struct_collection.contains_key(env_name) {
				let tex_type = clone_tex_type(self.tex_struct_collection.get(env_name).unwrap());
				let math_struct = TexStructure::new(String::from("NOLABEL"), tex_type);

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

		fn check_latexmk_macro(&mut self, line: &String) {
			match self.current_env {
				EnvEnumState::Proof => {

					// Not robust
					let vec = line.split(" ").collect::<Vec<&str>>();

					if vec.len() != 4 {
						return
					}

					else if vec[0] != "!TEX" {
						return
					}

					else if vec[1] != "proof" {
						return
					}

					else if vec[2] != "=" {
						return
					}

					let mut label = String::from(vec[3]);
					label = label.replace("{", "");
					label = label.replace("}", "");
					info!("Tex parser has found proof of {}", label);

					let mut proof = self.stack_proof.pop().unwrap();
					proof.set_struct_label(&label);
					self.stack_proof.push(proof);
				}

				_ => {
					// Do nothing
				}
			}
		}
	}
}

