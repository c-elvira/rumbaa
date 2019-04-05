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
	use std::io::prelude::*;

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

	#[derive(Debug)]
	enum TexParserState {
		Empty,
		InMacroName,
		InMacro,
		InMacroOptionalArg,
		InMacroArg,
	}

	struct TexParser<'a> {
		state: TexParserState,
		env_parser: EnvParser<'a>,
		bufcmd: String,
		stack_macro: Vec<TexCmd>,
	}

	impl<'a> TexParser<'a> {
		fn new(doc_input: &'a mut Document) -> Self {
	        TexParser {
	            state: TexParserState::Empty,
	            env_parser: EnvParser::new(doc_input),
	            bufcmd: String::from(""),
	            stack_macro: Vec::new(),
	        }
	    }

	    /**
	     * @brief State macine main logic
	     */
	    fn add_char(&mut self, c: char) {
	    	match self.state {

	    		TexParserState::Empty => {

	    			if c == '\\' {
	    				let texmacro = TexCmd::new(&String::from(""));
	    				self.stack_macro.push(texmacro);

	    				self.state = TexParserState::InMacroName;
			    	}
	    		}

	    		TexParserState::InMacroName => {

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
						self.set_macro_name_from_buf();
    					let tex_macro = self.stack_macro.pop().unwrap();
						self.env_parser.process_macro(tex_macro);

	    				if self.stack_macro.len() == 0 {
		    				self.state = TexParserState::Empty;
		    			}
					}

					else if c == '[' {
						self.set_macro_name_from_buf();
						self.state = TexParserState::InMacroOptionalArg;
					}

					else if c == '{' {
						self.set_macro_name_from_buf();
						self.state = TexParserState::InMacroArg;
					}

					else {
						self.set_macro_name_from_buf();
    					let tex_macro = self.stack_macro.pop().unwrap();
						self.env_parser.process_macro(tex_macro);

	    				if self.stack_macro.len() == 0 {
		    				self.state = TexParserState::Empty;
		    			}
					}
	    		}

	    		TexParserState::InMacro => {
	    			match c {
	    				'{' => {
	    					self.state = TexParserState::InMacroArg;
	    				}

	    				'[' => {
	    					self.state = TexParserState::InMacroOptionalArg;
	    				}

	    				_ => {
	    					let tex_macro = self.stack_macro.pop().unwrap();
	    					self.env_parser.process_macro(tex_macro);
	    					if self.stack_macro.len() == 0 {
		    					self.state = TexParserState::Empty;
		    				}
	    				}
	    			}
	    		}

	    		TexParserState::InMacroOptionalArg => {
	    			match c {
	    				']' => {
							let mut tex_macro = self.stack_macro.pop().unwrap();
							tex_macro.add_opt_arg(&self.bufcmd);
							self.stack_macro.push(tex_macro);

							self.bufcmd = String::from("");
							self.state = TexParserState::InMacro;
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
							self.state = TexParserState::InMacro;
	    				}

	    				_ => {
	    					self.bufcmd.push(c);
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

	enum EnvEnumState {
		None,
		Theorem, // definition, theorem, custom
		Proof,
		Equation,
		Other,
	}


	struct EnvParser<'a> {
		stack_env: Vec<EnvEnumState>,
		stack_theorem: Vec<TexStructure>,
		stack_proof: Vec<Proof>,
		tex_struct_collection: HashMap<String, EnumTexType>,
		equation_env_collection: Vec<String>,

		doc: &'a mut Document,
	}

	impl<'a> EnvParser<'a> {
		fn new(doc_input: &'a mut Document) -> Self {
	        EnvParser {
	            stack_env: vec![EnvEnumState::None],
	            stack_theorem: Vec::new(),
	            stack_proof: Vec::new(),
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
					// Command not supported
					// do nothing
				}
			}
		}

		fn open_env(&mut self, env_name: &String) {
			// 1. if is theorem, definition etc...
			if self.tex_struct_collection.contains_key(env_name) {
				let tex_type = clone_tex_type(self.tex_struct_collection.get(env_name).unwrap());
				let math_struct = TexStructure::new(String::from("NOLABEL"), tex_type);

				self.stack_theorem.push(math_struct);
				self.stack_env.push(EnvEnumState::Theorem);
			}

			else if self.equation_env_collection.contains(&env_name) {
				self.stack_env.push(EnvEnumState::Equation);
			}

			else if env_name == "proof" {
				let proof = Proof::new("NOTH".to_string());
				self.stack_proof.push(proof);

				self.stack_env.push(EnvEnumState::Proof);
			}

			else {
				// We don't care
				self.stack_env.push(EnvEnumState::Other);
			}
		}

		fn add_label_to_env(&mut self, label: String) {
			let tex_env = self.stack_env.pop().unwrap();

			match tex_env {
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
					// Add label to Env
				}

				EnvEnumState::Other => {
					// Do nothing
				}

				EnvEnumState::None => {
					// Do nothing
				}
			}

			self.stack_env.push(tex_env);
		}

		fn close_env(&mut self) {
			let tex_env = self.stack_env.pop().unwrap();

			match tex_env {
				EnvEnumState::Theorem => {
					let math_struct = self.stack_theorem.pop().unwrap();
					let label = math_struct.clone_label();
					if label != "NOLABEL" {
						self.doc.push(label, math_struct);
					}
				}

				EnvEnumState::Proof => {
					let proof = self.stack_proof.pop().unwrap();
					let label = proof.get_struct_label();

					if label != "NOTH" {
						self.doc.set_proof(&label, proof);
					}
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
		}
	}
}

