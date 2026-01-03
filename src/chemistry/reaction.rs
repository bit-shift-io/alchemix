use crate::chemistry::atom::Atom;

#[derive(Debug, Clone)]
pub struct MoleculeCandidate {
    pub atoms: Vec<Atom>,
}

impl MoleculeCandidate {
    pub fn name(&self) -> String {
        if self.atoms.is_empty() {
            return "Empty".to_string();
        }

        let mut counts = std::collections::HashMap::new();
        for atom in &self.atoms {
            *counts.entry(atom.element.symbol).or_insert(0) += 1;
        }

        let mut sorted_symbols: Vec<_> = counts.keys().collect();
        sorted_symbols.sort();

        let mut result = String::new();
        for &symbol in sorted_symbols {
            result.push_str(symbol);
            let count = counts[symbol];
            if count > 1 {
                result.push_str(&count.to_string());
            }
        }
        result
    }
}

pub fn bond_atoms(atoms: Vec<Atom>) -> Option<MoleculeCandidate> {
    if atoms.is_empty() {
        return None;
    }

    if atoms.len() == 1 {
        // Monatomic elements like Noble Gases can exist alone, 
        // but for "spell mixing" we might expect at least 2 atoms 
        // or a noble gas.
        let valency = atoms[0].element.valency();
        if valency == 0 {
            return Some(MoleculeCandidate { atoms });
        }
        return None;
    }

    let valencies: Vec<u8> = atoms.iter().map(|a| a.element.valency()).collect();
    let total_valency: u32 = valencies.iter().map(|&v| v as u32).sum();
    let max_valency = *valencies.iter().max().unwrap() as u32;

    // A group of atoms can form a molecule if:
    // 1. Total valency is even (each bond shares two valency slots)
    // 2. The most demanding atom can be satisfied by the others.
    if total_valency % 2 == 0 && max_valency <= (total_valency - max_valency) {
        Some(MoleculeCandidate { atoms })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chemistry::element::get_by_symbol;

    fn get_atom(sym: &str) -> Atom {
        Atom::new(get_by_symbol(sym).unwrap())
    }

    #[test]
    fn test_water_bonding() {
        let atoms = vec![get_atom("H"), get_atom("H"), get_atom("O")];
        let molecule = bond_atoms(atoms).expect("Should form H2O");
        assert_eq!(molecule.name(), "H2O");
    }

    #[test]
    fn test_carbon_dioxide_bonding() {
        let atoms = vec![get_atom("C"), get_atom("O"), get_atom("O")];
        let molecule = bond_atoms(atoms).expect("Should form CO2");
        assert_eq!(molecule.name(), "CO2");
    }

    #[test]
    fn test_noble_gas_bonding() {
        let atoms = vec![get_atom("He")];
        let molecule = bond_atoms(atoms).expect("Helium should be stable alone");
        assert_eq!(molecule.name(), "He");
    }

    #[test]
    fn test_impossible_bonding() {
        let atoms = vec![get_atom("H"), get_atom("C")];
        let molecule = bond_atoms(atoms);
        assert!(molecule.is_none(), "H and C alone cannot fully bond (needs 4 H for CH4)");
    }
}
