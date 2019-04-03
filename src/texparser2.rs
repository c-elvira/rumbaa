extern crate log;

use std::collections::HashMap;
use std::io::prelude::*;



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
		//let tex_doc = Document::new(main_filename.to_string());

			// Creating tex parser
	    let mut parser = TexParser::new(&main_filename);

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

		Ok(parser.get_doc())
	}

	#[derive(Debug)]
	enum TexParserState {
		Empty,
		InMacroName,
		InMacro,
		InMacroOptionalArg,
		InMacroArg,
	}

	struct TexParser {
		state: TexParserState,
		doc: Document,
		bufcmd: String,
		stack_macro: Vec<TexCmd>,
		stack_env: Vec<TexCmd>,
	}

	impl TexParser {
		fn new(filename: &String) -> Self {
	        TexParser {
	            state: TexParserState::Empty,
	            doc: Document::new(filename.to_string()),
	            bufcmd: String::from(""),
	            stack_macro: Vec::new(),
	            stack_env: Vec::new(),
	        }
	    }

	    fn get_doc(self) -> Document {
	    	return self.doc
	    }

	    /**
	     * @brief State macine main logic
	     * @details [long description]
	     * 
	     * @param self [description]
	     * @param r [description]
	     * 
	     * @return [description]
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
	    				self.process_macro();

	    				let texmacro = TexCmd::new(&String::from(""));
	    				self.stack_macro.push(texmacro);    				
	    			}
					else if c == ' ' {
						self.set_macro_name_from_buf();
	    				self.process_macro();

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
						self.process_macro();

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
	    					self.process_macro();
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

		fn process_macro(&mut self) {
			let tex_macro = self.stack_macro.pop().unwrap();

			println!("name: {}, args: {:?}, opt: {:?}", 
				tex_macro.name, tex_macro.args, tex_macro.option_args);
		}
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
}

