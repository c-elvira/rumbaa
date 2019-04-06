extern crate log;

pub mod texparser {	
	use std::fs::File;
	use std::io::{BufRead,BufReader};

	use crate::envparser::texparser::{EnvParser};
	use crate::texstruct::tex_logic::{EnumMacroType,TexMacro};
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

			// Creating tex env parser
		let mut env_parser = EnvParser::new(&mut tex_doc);

			// Creating tex parser
		let mut parser = TexParser::new();

		// 1. Reading
		let mut reader = BufReader::new(main_clean_file);
		let mut buf = Vec::<u8>::new();

		// -- starting reading loop
		while reader.read_until(b'\n', &mut buf).expect("read_until failed") != 0 {
			// this moves the ownership of the read data to s
			// there is no allocation
			let s = String::from_utf8(buf).expect("from_utf8 failed");
			for c in s.chars() {
				match parser.add_char(c) {
					Some(tex_macro) => {
						env_parser.process_macro(&tex_macro);
					}
					None => ()
				}
			}

			// this returns the ownership of the read data to buf
			// there is no allocation
			buf = s.into_bytes();
			buf.clear();
		}

		Ok(tex_doc)
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

	//struct TexParser<'a> {
	struct TexParser {
		//env_parser: EnvParser<'a>,
		current_state: TexParserState,
		stack_state: Vec<TexParserState>,
		stack_macro: Vec<TexMacro>,
		bufcmd: String,
		buf_comment: String,
	}

	//impl<'a> TexParser<'a> {
	impl TexParser {
		//fn new(doc_input: &mut Document) -> Self {
		fn new() -> Self {
			TexParser {
				current_state: TexParserState::Empty,
				stack_state: Vec::new(),
				//env_parser: EnvParser::new(doc_input),
				//process_macro: callback_macro,
				bufcmd: String::from(""),
				buf_comment: String::from(""),
				stack_macro: Vec::new(),
			}
		}

		/**
		 * @brief State macine main logic
		 */
		fn add_char(&mut self, c: char) -> Option<TexMacro> {

			let mut macro_out = None;

			match self.current_state {

				TexParserState::Empty => {
					match c {

						'\\' => {
							let texmacro = TexMacro::new(EnumMacroType::Tex);
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
						macro_out = Some(self.stack_macro.pop().unwrap());

						self.stack_macro.push(TexMacro::new(EnumMacroType::Tex));    				
					}
					else if c == ' ' {
						if self.bufcmd != "" {
							self.set_macro_name_from_buf();
						}
						macro_out = Some(self.stack_macro.pop().unwrap());

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

						macro_out =  Some(self.stack_macro.pop().unwrap());

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
							macro_out = self.parse_latexmk_macro();
							//self.env_parser.check_latexmk_macro(&self.buf_comment);
							// TODO

							self.buf_comment = "".to_string();
							self.current_state = self.stack_state.pop().unwrap();
						}

						_ => {
							self.buf_comment.push(c);
						}
					}
				}
			}

			macro_out
		}

		fn set_macro_name_from_buf(&mut self) {
			let mut tex_macro = self.stack_macro.pop().unwrap();
			tex_macro.set_name(&self.bufcmd);
			self.bufcmd = String::from("");

			self.stack_macro.push(tex_macro);
		}

		fn parse_latexmk_macro(&mut self) -> Option<TexMacro> {

			let mut macro_out = None;

			// Not robust
			let vec = self.buf_comment.split(" ").collect::<Vec<&str>>();

			if vec.len() != 4 {
				//
			}

			else if vec[0] != "!TEX" {
				//
			}

			else if vec[1] != "proof" {
				//
			}

			else if vec[2] != "=" {
				//
			}
			else {
				let mut label = String::from(vec[3]);
				label = label.replace("{", "");
				label = label.replace("}", "");

				let mut mk_macro = TexMacro::new(EnumMacroType::LatexMk);
				mk_macro.set_name(&"proof".to_string());
				mk_macro.add_arg(&label);

				macro_out = Some(mk_macro);
			}

			macro_out
		}
	}


	/* -------------------------------

				Tests

	------------------------------- */


	#[cfg(test)]
	mod tests {

		use crate::texparser2::texparser::{TexParser};

		#[test]
		fn one_simple_macro() {
			let tex_line = String::from("\\mymacro[opt]{arg1}{arg2}");
			let mut parser = TexParser::new();

			// 1. Assert that nothing is returned
			for c in tex_line.chars() {
				assert!(parser.add_char(c).is_none())
			}

			// 2. Assert end of macro
			let opt_macro_out = parser.add_char('\n');
			assert!(!opt_macro_out.is_none());

			// 3. Check argument
			let macro_out = opt_macro_out.unwrap();
			assert!(macro_out.get_nb_args() == 2);
			assert!(macro_out.get_arg(0) == String::from("arg1"));
			assert!(macro_out.get_arg(1) == String::from("arg2"));

			assert!(macro_out.get_nb_opt_args() == 1);
			assert!(macro_out.get_opt_arg(0) == String::from("opt"));
		}

	}
}

