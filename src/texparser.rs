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
		InMacroName,
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
		current_buffer: String,
		stack_buffer: Vec<String>,
		buf_comment: String,
	}

	//impl<'a> TexParser<'a> {
	impl TexParser {
		//fn new(doc_input: &mut Document) -> Self {
		fn new() -> Self {
			TexParser {
				current_state: TexParserState::Empty,
				stack_state: Vec::new(),
				current_buffer: String::from(""),
				stack_buffer: Vec::new(),
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
							self.start_new_macro();
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

				TexParserState::InMacroName => {
					if c.is_alphabetic() {
						// still in macro name
						self.current_buffer.push(c);
					}

					else if c == '\\' {
						if self.current_buffer == "" {
							// in fact we are handling \newline command
							// delete previously created macro

							self.current_state = self.stack_state.pop().unwrap();
						}
						else {
							// Start another macro
							macro_out = Some(self.create_macro_from_buf());
						}
					}

					else if c == '[' {
						if self.current_buffer != "" {
							let new_macro = self.create_macro_from_buf();

							self.stack_macro.push(new_macro);
							self.current_state = TexParserState::InMacroOptionalArg;
						}
						else {
							// Not implemented yet
						}
					}

					else if c == '{' {
						if self.current_buffer != "" {
							let new_macro = self.create_macro_from_buf();

							self.stack_macro.push(new_macro);
							self.current_state = TexParserState::InMacroArg;
						}
						else {
							// Not implemented yet
						}
					}

					else {
						// Macro ends without argument
						if self.current_buffer != "" {
							macro_out = Some(self.create_macro_from_buf());
							self.current_state = self.stack_state.pop().unwrap();

							match self.add_char(c) {
								None => {
									// Ok
								}

								Some(_) => {
									// This should not happen
									println!("this should not happen");
								}
							}	
						}
						else {
							// Not implemented yet
						}
					}
				}

				TexParserState::InMacro => {

					if c == '[' {
						self.current_state = TexParserState::InMacroOptionalArg;
					}

					else if c == '{' {
						self.current_state = TexParserState::InMacroArg;
					}

					else if c == '%' {
						// Macro ends
						macro_out = Some(self.stack_macro.pop().unwrap());
						self.current_state = TexParserState::InComment;
					}

					else if c == '}' || c == ']' {
						// In this case, we are inside nested macro
						// close the inner macro, tell the outer arg ends

						let ended_macro = self.stack_macro.pop().unwrap();
						self.current_buffer = self.stack_buffer.pop().unwrap();
						self.current_buffer += &ended_macro.get_tex_code();

						macro_out = Some(ended_macro);
						self.current_state = self.stack_state.pop().unwrap();

						match self.add_char(c) {
							None => {
								// Ok
							}

							Some(_) => {
								// This should not happen
								println!("this should not happen");
							}
						}
					}

					else {
						macro_out =  Some(self.stack_macro.pop().unwrap());
						self.current_state = self.stack_state.pop().unwrap();
					}
				}

				TexParserState::InMacroOptionalArg => {
					match c {
						']' => {
							let mut tex_macro = self.stack_macro.pop().unwrap();
							tex_macro.add_opt_arg(&self.current_buffer);
							self.stack_macro.push(tex_macro);

							self.current_buffer = String::from("");
							self.current_state = TexParserState::InMacro;
						}

						'\\' => {
							// Start new macro
							self.start_new_macro();
						}

						'%' => {
							self.stack_state.push(self.current_state.clone());
							self.current_state = TexParserState::InComment;
						}

						_ => {
							self.current_buffer.push(c);
						}
					}
				}

				TexParserState::InMacroArg => {
					match c {
						'}' => {
							let mut tex_macro = self.stack_macro.pop().unwrap();
							tex_macro.add_arg(&self.current_buffer);
							self.stack_macro.push(tex_macro);

							self.current_buffer = String::from("");
							self.current_state = TexParserState::InMacro;
						}

						'\\' => {
							// Start new macro
							self.start_new_macro();
						}

						'%' => {
							self.stack_state.push(self.current_state.clone());
							self.current_state = TexParserState::InComment;
						}

						_ => {
							self.current_buffer.push(c);
						}
					}
				}

				TexParserState::InComment => {
					match c {
						'\n' => {
							// End of comment, process it
							macro_out = self.parse_latexmk_macro();

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

		fn start_new_macro(&mut self) {
			self.stack_state.push(self.current_state.clone());

			self.stack_buffer.push(self.current_buffer.clone());
			self.current_buffer = String::from("");

			self.current_state = TexParserState::InMacroName;
		}

		fn create_macro_from_buf(&mut self) -> TexMacro {
			let mut tex_macro = TexMacro::new(EnumMacroType::Tex);

			let clean_name = self.current_buffer.replace(" ", "");
			let clean_name = clean_name.replace("\t", "");
			let clean_name = clean_name.replace("\n", "");
			tex_macro.set_name(&clean_name);

			self.current_buffer = String::from("");

			tex_macro
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

		use crate::texparser::texparser::{TexParser};

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

		#[test]
		fn nested_macros() {
			let tex_line_part1 = String::from("\\macroOne{\\macroTwo{arg}");
			let tex_line_part2 = '}';
			let tex_line_part3 = '\n';
			let mut parser = TexParser::new();

			// 1. Assert that nothing is returned
			for c in tex_line_part1.chars() {
				assert!(parser.add_char(c).is_none())
			}

			// 2. End of first macro
			let opt_macro_out_1 = parser.add_char(tex_line_part2);

				// 2.1
			assert!(!opt_macro_out_1.is_none());

				// 2.2. Check argument
			let macro_out_1 = opt_macro_out_1.unwrap();
			assert!(macro_out_1.get_nb_args() == 1);
			assert!(macro_out_1.get_arg(0) == String::from("arg"));

			// 3. End of second macro
			let opt_macro_out_2 = parser.add_char(tex_line_part3);

				// 2.1
			assert!(!opt_macro_out_2.is_none());

				// 2.2. Check argument
			let macro_out_2 = opt_macro_out_2.unwrap();
			assert!(macro_out_2.get_nb_args() == 1);
			assert!(macro_out_2.get_arg(0) == String::from("\\macroTwo{arg}"));
		}

		#[test]
		fn two_nested_macro() {	
			let tex_line_1 = String::from("\\kvbar{\\kangle{\\atome");
			let tex_line_2 = '(';
			let tex_line_3 = String::from("\\param");
			let tex_line_4 = ')';
			let tex_line_5 = String::from(",\\Vobs");
			let tex_line_6 = '}';
			let tex_line_7 = '}';
			let tex_line_8 = '\n';

			let mut parser = TexParser::new();

			// 1. Assert that nothing is returned
			for c in tex_line_1.chars() {
				assert!(parser.add_char(c).is_none())
			}

			// 2. End of first macro
			let opt_macro_out_1 = parser.add_char(tex_line_2);
			{
				assert!(!opt_macro_out_1.is_none());

				let macro_out = opt_macro_out_1.unwrap();
				assert!(macro_out.get_name() == String::from("atome"));
				assert!(macro_out.get_nb_args() == 0);
			}

			// 3. nothing happen
			for c in tex_line_3.chars() {
				assert!(parser.add_char(c).is_none())
			}

			// 4. End of second macro
			let opt_macro_out_2 = parser.add_char(tex_line_4);
			{
				assert!(!opt_macro_out_2.is_none());

				let macro_out = opt_macro_out_2.unwrap();
				assert!(macro_out.get_name() == String::from("param"));
				assert!(macro_out.get_nb_args() == 0);
			}

			// 5. nothing happen
			for c in tex_line_5.chars() {
				assert!(parser.add_char(c).is_none())
			}

			// 6. End of second macro
			let opt_macro_out_3 = parser.add_char(tex_line_6);
			{
				assert!(!opt_macro_out_3.is_none());

				let macro_out = opt_macro_out_3.unwrap();
				assert!(macro_out.get_name() == String::from("Vobs"));
				assert!(macro_out.get_nb_args() == 0);
			}

			// 7. End of second macro
			let opt_macro_out_4 = parser.add_char(tex_line_7);
			{
				assert!(!opt_macro_out_4.is_none());

				let macro_out = opt_macro_out_4.unwrap();
				assert!(macro_out.get_name() == String::from("kangle"));
				assert!(macro_out.get_nb_args() == 1);
			}

			// 8. End of second macro
			let opt_macro_out_5 = parser.add_char(tex_line_8);
			{
				assert!(!opt_macro_out_5.is_none());

				let macro_out = opt_macro_out_5.unwrap();
				assert!(macro_out.get_name() == String::from("kvbar"));
				assert!(macro_out.get_nb_args() == 1);
			}
		}

		#[test]
		fn clean_macro_name() {
			//todo: remove white space in nested macro name
		}

		#[test]
		fn handle_breakline() {
			//todo: what happens when "//" is met?
			let tex_line_part1 = String::from("\\\\");

			let mut parser = TexParser::new();

			for c in tex_line_part1.chars() {
				assert!(parser.add_char(c).is_none())
			}
		}
	}
}

