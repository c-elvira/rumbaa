/* Tex structure */

pub struct Definition {
	label: String,
	name: String,
	proof: Option<Proof>,
	ilabel: i32,
	page: i32,
}

pub struct Theorem {
	label: String,
	name: String,
	proof: Option<Proof>,
	ilabel: i32,
	page: i32,
}

pub struct Proposition {
	label: String,
	name: String,
	proof: Option<Proof>,
	ilabel: i32,
	page: i32,
}

pub struct Lemma {
	label: String,
	name: String,
	proof: Option<Proof>,
	ilabel: i32,
	page: i32,
}

// Todo: TexStructFactory



/* Constructors */

impl Definition{
	pub fn new (label:String) -> Self {
		Self {
			label: label,
			name: String::from("None"),
			proof: None,
			ilabel: 0,
			page: 0,
		}
	}
}

impl Theorem{
	pub fn new (label:String) -> Self {
		Self {
			name: String::from("None"),
			label: label,
			proof: None,
			ilabel: 0,
			page: 0,
		}
	}
}

impl Proposition{
	pub fn new (label:String) -> Self {
		Self {
			name: String::from("None"),
			label: label,
			proof: None,
			ilabel: 0,
			page: 0,
		}
	}
}

impl Lemma{
	pub fn new (label:String) -> Self {
		Self {
			name: String::from("None"),
			label: label,
			proof: None,
			ilabel: 0,
			page: 0,
		}
	}
}

/* trait TexStructure */

pub trait TexStructure {
	// add code here

	fn print(&self) -> String;

	fn set_proof(&mut self, proof: Proof);
	fn set_ilabel(&mut self, ilabel: i32);
	fn set_page(&mut self, page: i32);
	fn set_name(&mut self, name: String);


	fn get_proof(&self) -> &Option<Proof>;
	fn get_name(&self) -> &String;
}

impl TexStructure for Definition {
	
	fn print(&self) -> String {
		let output = " - Definition".to_owned() 
			+ ": " + &self.label
			+ &self.name;

		output
	}

	fn set_proof(&mut self, proof: Proof) {
		self.proof = Some(proof);
	}

	fn set_ilabel(&mut self, ilabel: i32) {
		self.ilabel = ilabel;
		self.name   = String::from("Def. ".to_owned() + &self.ilabel.to_string());
	}

	fn set_page(&mut self, page: i32) {
		self.page = page;
	}

	fn set_name(&mut self, name: String) {
		self.name = name;
	}

	fn get_proof(&self) -> &Option<Proof> {
		&self.proof
	}

	fn get_name(&self) -> &String {
		match self.name.as_ref() {
			"None" => return &self.label,
			_ => return &self.name,
		}
	}
}

impl TexStructure for Theorem {

	fn print(&self) -> String {
		let output = " - Theorem".to_owned()
			+ ": " + &self.label 
			+ &self.name; // + &self.text;

		output
	}

	fn set_proof(&mut self, proof: Proof) {
		self.proof = Some(proof);
	}

	fn set_ilabel(&mut self, ilabel: i32) {
		self.ilabel = ilabel;
		self.name   = String::from("Th. ".to_owned() + &self.ilabel.to_string());
	}

	fn set_page(&mut self, page: i32) {
		self.page = page;
	}

	fn set_name(&mut self, name: String) {
		self.name = name;
	}

	fn get_proof(&self) -> &Option<Proof> {
		&self.proof
	}

	fn get_name(&self) -> &String {
		match self.name.as_ref() {
			"None" => return &self.label,
			_ => return &self.name,
		}
	}
}

impl TexStructure for Proposition {

	fn print(&self) -> String {
		let output = " - Proposition".to_owned()
			+ ": " + &self.label 
			+ &self.name; // + &self.text;

		output
	}

	fn set_proof(&mut self, proof: Proof) {
		self.proof = Some(proof);
	}

	fn set_ilabel(&mut self, ilabel: i32) {
		self.ilabel = ilabel;
		self.name   = String::from("Prop. ".to_owned() + &self.ilabel.to_string());
	}

	fn set_page(&mut self, page: i32) {
		self.page = page;
	}

	fn set_name(&mut self, name: String) {
		self.name = name;
	}

	fn get_proof(&self) -> &Option<Proof> {
		&self.proof
	}

	fn get_name(&self) -> &String {
		match self.name.as_ref() {
			"None" => return &self.label,
			_ => return &self.name,
		}
	}
}

impl TexStructure for Lemma {

	fn print(&self) -> String {
		let output = " - Lemma".to_owned()
			+ ": " + &self.label 
			+ &self.name; // + &self.text;

		output
	}

	fn set_proof(&mut self, proof: Proof) {
		self.proof = Some(proof);
	}

	fn set_ilabel(&mut self, ilabel: i32) {
		self.ilabel = ilabel;

		if self.name == String::from("None") {
			self.name = String::from("Lem. ".to_owned() + &self.ilabel.to_string());
		} 
	}

	fn set_page(&mut self, page: i32) {
		self.page = page;
	}

	fn set_name(&mut self, name: String) {
		self.name = name;
	}

	fn get_proof(&self) -> &Option<Proof> {
		&self.proof
	}

	fn get_name(&self) -> &String {
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