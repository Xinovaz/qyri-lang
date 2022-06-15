use crate::parsing::AstNode;

/* DEF : Qyri Scope { public }
 * Top-level program data
 * Stores all variables, abstractions, and code
 */
#[derive(Debug)]
pub struct QyriScope {

	/* DEF : Initialisation Script 1 { private }
	 * Used for built-in code execution
	 */
	init_: Vec<QyriInstruction>,

	/* DEF : Initialisation Script 2 { public }
	 * Used for auto-imported std code execution and struct derivation
	 */
	pub init__: Vec<QyriInstruction>,

	/* DEF : Cardinal Script { public }
	 * Main code execution
	 */
	pub card: Vec<QyriInstruction>,

	/* DEF : Ordinal Scripts { private }
	 * Post-cardinal code execution
	 */
	ord: Vec<Vec<QyriInstruction>>,


}
/* IMPL */
impl QyriScope {
	fn default() -> QyriScope {
		QyriScope {
			init_: vec![],
			init__: vec![],
			card: vec![],
			ord: vec![],
		}
	}
}


/* DEF : Qyri Instruction { public }
 * Representative of a single Qyri Instruction
 */
#[derive(Debug)]
pub enum QyriInstruction {
	DebugNop,
}

/* DEF : Qyri Atom { public }
 * Representative of a primitive type / binary data
 */
 #[derive(Debug)]
pub struct QyriAtom<'a>(&'a [u8]);
/* IMPL */
impl<'a> QyriAtom<'a> {
	/* DEF : Qyri Atom Constructor { public }
	 * Create a Qyri Atom
	 */
	pub fn new(a: &'a [u8]) -> QyriAtom<'a> {
		QyriAtom(a)
	}
}

/* DEF : Qyri Primitive Type { public }
 * Representative of a primitive value
 */
pub enum QyriPrimitive<'a> {
	/* DEF : 8-bit unsigned integer { public } */
	byte_(QyriAtom<'a>),

	/* DEF : 16-bit unsigned integer { public } */
	word_(QyriAtom<'a>),

	/* DEF : 32-bit unsigned integer { public } */
	long_(QyriAtom<'a>),

	/* DEF : 32-bit signed integer { public } */
	int_(QyriAtom<'a>),

	/* DEF : 16-bit floating point number { public } */
	float_(QyriAtom<'a>),

	/* DEF : 32-bit floating point number { public } */
	double_(QyriAtom<'a>),

	/* DEF : 8-bit boolean value { public } */
	bool_(QyriAtom<'a>),

	/* DEF : (unused) 64-bit pure binary value { public } */
	atomic_(QyriAtom<'a>),
}

/* DEF : Qyri Abstraction { public trait }
 * Struct or enum
 */
pub trait QyriData {
	/* DEF : Get Abstraction Data { public } */
	fn get_data(&self) -> &Vec<Box<QyriType<'static>>>;

	/* DEF : Get Abstraction Data Mutably { public } */
	fn get_data_mut(&mut self) -> &mut Vec<Box<QyriType<'static>>>;

	/* DEF : Get Abstraction Scope { public } */
	fn get_env(&self) -> &Box<QyriScope>;

	/* DEF : Get Abstraction Scope Mutably { public } */
	fn get_env_mut(&mut self) -> &mut Box<QyriScope>;
}

/* DEF : Null Qyri Abstraction { private }
 * Used for QyriType<T: QyriData> where the type is primitive
 */
struct NullData {
	/* DEF : Null Data List { private }
	 * Used only for the QyriData trait, which requires references
	 */
	l: Vec<Box<QyriType<'static>>>,
	/* DEF : Null Data Scope { private }
	 * Used only for the QyriData trait, which requires references
	 */
	s: Box<QyriScope>,
}
/* IMPL */
impl NullData {
	/* DEF : Null Data Constructor { public }
	 * Create some null data
	 */
	fn default() -> NullData {
		NullData {
			l: vec![],
			s: Box::new(QyriScope::default()),
		}
	}
}
/* IMPL */
impl QyriData for NullData {
	fn get_data(&self) -> &Vec<Box<QyriType<'static>>> {
		&self.l
	}

	fn get_data_mut(&mut self) -> &mut Vec<Box<QyriType<'static>>> {
		&mut self.l
	}

	fn get_env(&self) -> &Box<QyriScope> {
		&self.s
	}

	fn get_env_mut(&mut self) -> &mut Box<QyriScope> {
		&mut self.s
	}
}

/* DEF : Qyri Enumeration { public }
 * Enum
 */
pub struct QyriEnumeration {
	/* DEF : Enumeration Variants { public }
	 * Possible variants of the enum
	 */
	pub variants: Vec<Box<QyriType<'static>>>,

	/* DEF : Enumeration Implementation { public }
	 * All associated methods of the abstraction
	 */
	pub implementation: Box<QyriScope>,
}
/* IMPL */
impl QyriEnumeration {
	/* DEF : Qyri Enumeration Constructor { public }
	 * Create a new enum from a list of variants
	 */
	pub fn new(variants: Vec<QyriType<'static>>) -> QyriEnumeration {
		let mut buf: Vec<Box<QyriType<'static>>> = Vec::new();
		for variant in variants {
			buf.push(Box::new(variant));
		}
		QyriEnumeration {
			variants: buf,
			implementation: Box::new(QyriScope::default()),
		}
	}
}
/* IMPL */
impl QyriData for QyriEnumeration {
	fn get_data(&self) -> &Vec<Box<QyriType<'static>>> {
		&self.variants
	}

	fn get_data_mut(&mut self) -> &mut Vec<Box<QyriType<'static>>> {
		&mut self.variants
	}

	fn get_env(&self) -> &Box<QyriScope> {
		&self.implementation
	}

	fn get_env_mut(&mut self) -> &mut Box<QyriScope> {
		&mut self.implementation
	}
}

/* DEF : Qyri Structure { public }
 * Struct
 */
pub struct QyriStructure {
	/* DEF : Structure Fields { public }
	 * Attributes of the data structure
	 */
	pub fields: Vec<Box<QyriType<'static>>>,

	/* DEF : Structure Implementation { public }
	 * All associated methods of the abstraction
	 */
	pub implementation: Box<QyriScope>,
}
/* IMPL */
impl QyriData for QyriStructure {
	fn get_data(&self) -> &Vec<Box<QyriType<'static>>> {
		&self.fields
	}

	fn get_data_mut(&mut self) -> &mut Vec<Box<QyriType<'static>>> {
		&mut self.fields
	}

	fn get_env(&self) -> &Box<QyriScope> {
		&self.implementation
	}

	fn get_env_mut(&mut self) -> &mut Box<QyriScope> {
		&mut self.implementation
	}
}

pub enum QyriType<'a> {
	primitive_(Box<QyriPrimitive<'a>>),
	enumeration_(Box<QyriEnumeration>),
	structure_(Box<QyriStructure>),
}