use crate::atom;

#[derive(Clone, Default, Debug)]
pub struct Bond {
    pub atoms: [std::cell::RefCell<atom::Atom>; 2],
}

impl Bond {
    pub fn new(atoms: [std::cell::RefCell<atom::Atom>; 2]) -> Self {
        Self { atoms }
    }
}
