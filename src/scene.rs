use crate::{atom::Atom, bond::Bond};

#[derive(Default)]
pub struct Scene {
    pub atoms: Vec<Atom>,
    pub bonds: Vec<Bond>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            atoms: Vec::new(),
            bonds: Vec::new(),
        }
    }
    pub fn add_atom(&mut self, atom: Atom) {
        self.atoms.push(atom);
    }
    pub fn add_bond(&mut self, bond: Bond) {
        self.bonds.push(bond);
    }
}
