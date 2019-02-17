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
}

/* Tex structure */

pub struct Definition {
	name: String,
	label: String,
	text: String,
}

pub struct Proof {
	text: String,
}

pub struct Theorem {
	name: String,
	proof: Proof,
	label: String,
	text: String,
}

pub struct Proposition {
	name: String,
	proof: Proof,
	label: String,
	text: String,
}

pub struct Lemma {
	name: String,
	proof: Proof,
	label: String,
	text: String,
}

// Todo: TexStructFactory



/* Constructors */

impl Definition{
	pub fn new (label:String, text: String) -> Self {
        Self {
        	name: String::from(""),
            label: label,
            text: text,
        }
	}
}

impl Proof {
	pub fn new (text: String) -> Self {
        Self {
            text: text,
        }
	}
}

impl Theorem{
	pub fn new (label:String, text: String) -> Self {
        Self {
        	name: String::from(""),
            label: label,
            text: text,
            proof: Proof::new(String::from("")),
        }
	}
}

/* trait TexStructure */

pub trait TexStructure {
	// add code here

	fn print(&self) -> String;
}

impl TexStructure for Definition {
	fn print(&self) -> String {
		let output = " - Definition".to_owned() 
			+ ": " + &self.label
			+ &self.name; // + &self.text;

		output
	}
}

impl TexStructure for Theorem {
	fn print(&self) -> String {
		let output = " - Theorem".to_owned()
			+ ": " + &self.label 
			+ &self.name; // + &self.text;

		output
	}
}