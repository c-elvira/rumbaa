/* Tex structure */

pub mod tex_logic {

	/* -----------------------------------------

					Tex Macro

	----------------------------------------- */

	#[derive(Debug, Clone)]
	pub enum EnumMacroType {
		Tex = 0,
		LatexMk,
	}

	#[derive(Debug)]
	pub struct TexMacro {
		name: String,
		macro_type: EnumMacroType,
		args: Vec<String>,
		option_args: Vec<String>,
	}

	impl TexMacro {
		pub fn new (macro_type: EnumMacroType) -> Self {
			Self {
				name: String::from(""),
				macro_type: macro_type,
				args: Vec::new(),
				option_args: Vec::new(),
			}
		}

		pub fn get_macro_type(&self) -> EnumMacroType {
			self.macro_type.clone()
		}

		pub fn get_name(&self) -> String {
			self.name.clone()
		}

		#[allow(dead_code)]
		pub fn get_nb_args(&self) -> usize {
			self.args.len()
		}

		pub fn get_nb_opt_args(&self) -> usize {
			self.option_args.len()
		}

		pub fn get_args(&self) -> Vec<String> {
			self.args.clone()
		}

		pub fn get_arg(&self, i: usize) -> String {
			self.args[i].clone()
		}

		pub fn get_opt_arg(&self, i: usize) -> String {
			self.option_args[i].clone()
		}

		pub fn get_tex_code(&self) -> String {
			let mut out = String::from("\\".to_owned() + &self.name);

			for arg in &self.option_args {
				out += &format!("[{}]", arg);
			}

			for arg in &self.args {
				out += &format!("{}{}{}", '{', arg, '}');
			}

			out
		}

		pub fn set_name(&mut self, cmd_name: &String) {
			self.name = cmd_name.clone();
		}

		pub fn add_arg(&mut self, arg: &String) {
			self.args.push(arg.clone());
		}

		#[allow(dead_code)]
		pub fn add_opt_arg(&mut self, opt_arg: &String) {
			self.option_args.push(opt_arg.clone());
		}
	}


	/* -----------------------------------------

					Theorem

	----------------------------------------- */

	#[derive(Copy, Clone)]
	pub enum EnumTheoremType {
		Custom = 0,
		Definition,
		Theorem,
		Proposition,
		Lemma,
		Corollary
	}

	pub struct Theorem {
		label: String,
		name: String,
		theorem_type: EnumTheoremType,
		proof: Option<Proof>,
		equation_labels: Vec<String>,
		ilabel: i32,
		page: i32,
	}

	impl Theorem {
		pub fn new (label:String, theorem_type: EnumTheoremType) -> Self {
			Self {
				label: label,
				name: String::from("None"),
				theorem_type: theorem_type,
				proof: None,
				equation_labels: Vec::new(),
				ilabel: 0,
				page: 0,
			}
		}
		
		pub fn add_equation(&mut self, eq_label: String) {
			self.equation_labels.push(eq_label);
		}

		pub fn contains_equation(&self, eq_label: &String) -> bool {
			self.equation_labels.contains(eq_label)
		}

		#[allow(dead_code)]
		pub fn print(&self) -> String {

			let rtype = match self.theorem_type {
				EnumTheoremType::Definition => " - Definition",
				EnumTheoremType::Theorem => " - Theorem",
				EnumTheoremType::Proposition => " - Proposition",
				EnumTheoremType::Lemma => " - Lemma",
				EnumTheoremType::Corollary => " - Corollary",
				EnumTheoremType::Custom => " - Other",
			};

			let output = rtype.to_owned() 
				+ ": " + &self.label
				+ &self.name;

			output
		}

		pub fn set_label(&mut self, new_label: &String) {
			self.label = new_label.clone();
		}

		pub fn set_proof(&mut self, proof: Proof) {
			self.proof = Some(proof);
		}

		pub fn set_ilabel(&mut self, ilabel: i32) {
			self.ilabel = ilabel;
			let type_str = match self.theorem_type {
				EnumTheoremType::Definition => "Def. ",
				EnumTheoremType::Theorem => " - Th. ",
				EnumTheoremType::Proposition => "Prop. ",
				EnumTheoremType::Lemma => "Lem. ",
				EnumTheoremType::Corollary => "Cor. ",
				EnumTheoremType::Custom => "Other ",	
			};
			self.name   = String::from(type_str.to_owned() + &self.ilabel.to_string());
		}

		pub fn set_page(&mut self, page: i32) {
			self.page = page;
		}

		pub fn set_name(&mut self, name: String) {
			self.name = name;
		}

		pub fn get_proof(&self) -> &Option<Proof> {
			&self.proof
		}

		pub fn get_name(&self) -> &String {
			match self.name.as_ref() {
				"None" => return &self.label,
				_ => return &self.name,
			}
		}

		#[allow(dead_code)]
		pub fn get_group(&self) -> i32 {
			let out :i32 = self.theorem_type.clone() as i32;

			out
		}

		pub fn clone_label(&self) -> String {
			return self.label.clone()
		}
	}

	
	/* -----------------------------------------

					Proof

	----------------------------------------- */

	pub struct Proof {
		struct_label: String,
		links: Vec<String>,

		title: String,
	}

	impl Proof {
		pub fn new (label: String) -> Self {
			Self {
				struct_label: label,
				links: Vec::new(),

				title: String::from("Proof"),
			}
		}

		pub fn add_link(&mut self, link: String) {
			self.links.push(link);
		}

		pub fn get_nblinks(&self) -> usize {
			self.links.len()
		}

		pub fn get_link(&self, i: usize) -> &String {
			&self.links[i]
		}

		pub fn get_struct_label(&self) -> String {
			self.struct_label.clone()
		}

		pub fn get_title(&self) -> &String {
			&self.title
		}

		pub fn set_struct_label(&mut self, label: &String) {
			self.struct_label = label.clone();
		}

		pub fn set_title(&mut self, arg: &String) {
			self.title = arg.clone();
		}
	}
}