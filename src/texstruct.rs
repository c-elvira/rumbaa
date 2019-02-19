use std::collections::HashMap;

pub struct Document {
	pub filename: String,
	pub list_tex_struct: HashMap<String, Box<TexStructure>>,
}

impl Document {

	pub fn new (strfilename: String) -> Self {
		Self {
			filename: strfilename,
			list_tex_struct: HashMap::new(),
		}
	}
	
	pub fn push<S: TexStructure + 'static>(&mut self, key: String, tex: S) -> &mut Self {
		self.list_tex_struct.insert(key, Box::new(tex));
		
		// return self
		self
	}

	pub fn print(&self) -> String {

		let mut output = String::from("Printing tex structure\n");


		for (_keylabel, tex_struct) in &self.list_tex_struct {
			output = output 
				+ &tex_struct.print()
				+ "\n";
		}

		output
	}

	pub fn keys(&self) -> std::collections::hash_map::Keys<'_, String, Box<TexStructure>> {

		self.list_tex_struct.keys()
	}

	pub fn get(&self, key: String) -> &Box<TexStructure> {

		&self.list_tex_struct[&key]
	}

	pub fn add_proof(&mut self, structlabel: &String, proof: Proof) {

		let texstruct = self.list_tex_struct.get_mut(structlabel).unwrap();
		texstruct.add_proof(proof);
	}
}

/* Tex structure */

pub struct Definition {
	label: String,
	name: String,
	proof: Option<Proof>,
}

pub struct Theorem {
	label: String,
	name: String,
	proof: Option<Proof>,
}

pub struct Proposition {
	label: String,
	name: String,
	proof: Option<Proof>,
}

pub struct Lemma {
	label: String,
	name: String,
	proof: Option<Proof>,
}

// Todo: TexStructFactory



/* Constructors */

impl Definition{
	pub fn new (label:String) -> Self {
		Self {
			label: label,
			name: String::from(""),
			proof: None,
		}
	}
}


impl Theorem{
	pub fn new (label:String) -> Self {
		Self {
			name: String::from(""),
			label: label,
			proof: None,
		}
	}
}

/* trait TexStructure */

pub trait TexStructure {
	// add code here

	fn print(&self) -> String;

	fn add_proof(&mut self, proof: Proof);

	fn get_proof(&self) -> &Option<Proof>;
}

impl TexStructure for Definition {
	
	fn print(&self) -> String {
		let output = " - Definition".to_owned() 
			+ ": " + &self.label
			+ &self.name;

		output
	}

	fn add_proof(&mut self, proof: Proof) {
		self.proof = Some(proof);
	}

	fn get_proof(&self) -> &Option<Proof> {

		&self.proof
	}
}

impl TexStructure for Theorem {

	fn print(&self) -> String {
		let output = " - Theorem".to_owned()
			+ ": " + &self.label 
			+ &self.name; // + &self.text;

		output
	}

	fn add_proof(&mut self, proof: Proof) {
		self.proof = Some(proof);
	}

	fn get_proof(&self) -> &Option<Proof> {

		&self.proof
	}
}


pub struct Proof {
	structlabel: String,
	links: Vec<String>,
}

impl Proof {
	pub fn new (structlabel: String) -> Self {
		Self {
			structlabel: structlabel,
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