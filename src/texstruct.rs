/* Tex structure */


pub enum EnumTexType {
	Definition,
	Theorem,
	Proposition,
	Lemma,
	Corollary,
	Other
}

pub fn clone_tex_type(tex_type: &EnumTexType) -> EnumTexType {
		match tex_type {
			EnumTexType::Definition  => EnumTexType::Definition,
			EnumTexType::Theorem 	 => EnumTexType::Theorem,
			EnumTexType::Proposition => EnumTexType::Proposition,
			EnumTexType::Lemma 		 => EnumTexType::Lemma,
			EnumTexType::Corollary   => EnumTexType::Corollary,
			_ 						 => EnumTexType::Definition,
		}
	}


pub struct TexStructure {
	label: String,
	name: String,
	math_type: EnumTexType,
	proof: Option<Proof>,
	ilabel: i32,
	page: i32,
}

impl TexStructure {
	pub fn new (label:String, math_type: EnumTexType) -> Self {
		Self {
			label: label,
			name: String::from("None"),
			math_type: math_type,
			proof: None,
			ilabel: 0,
			page: 0,
		}
	}
	
	pub fn print(&self) -> String {

		let rtype = match self.math_type {
			EnumTexType::Definition => " - Definition",
			EnumTexType::Theorem => " - Theorem",
			EnumTexType::Proposition => " - Proposition",
			EnumTexType::Lemma => " - Lemma",
			EnumTexType::Corollary => " - Corollary",
			EnumTexType::Other => " - Other",
		};

		let output = rtype.to_owned() 
			+ ": " + &self.label
			+ &self.name;

		output
	}

	pub fn set_proof(&mut self, proof: Proof) {
		self.proof = Some(proof);
	}

	pub fn set_ilabel(&mut self, ilabel: i32) {
		self.ilabel = ilabel;
		let type_str = match self.math_type {
			EnumTexType::Definition => "Def. ",
			EnumTexType::Theorem => " - Th. ",
			EnumTexType::Proposition => "Prop. ",
			EnumTexType::Lemma => "Lem. ",
			EnumTexType::Corollary => "Cor. ",
			EnumTexType::Other => "Other ",	
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
}


pub struct Proof {
	_structlabel: String,
	links: Vec<String>,
}

impl Proof {
	pub fn new (structlabel: String) -> Self {
		Self {
			_structlabel: structlabel,
			links: Vec::new(),
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
}