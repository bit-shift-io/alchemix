#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Element {
    pub atomic_number: u8,
    pub symbol: &'static str,
    pub name: &'static str,
}

impl Element {
    pub const fn new(atomic_number: u8, symbol: &'static str, name: &'static str) -> Self {
        Self {
            atomic_number,
            symbol,
            name,
        }
    }

    pub fn shells(&self) -> Vec<u8> {
        let mut electrons = self.atomic_number;
        let mut shells = Vec::new();
        let shell_capacities = [2, 8, 8, 18, 18, 32, 32];

        for &capacity in &shell_capacities {
            if electrons == 0 {
                break;
            }
            let taking = std::cmp::min(electrons, capacity);
            shells.push(taking);
            electrons -= taking;
        }
        shells
    }

    pub fn valence_electrons(&self) -> u8 {
        let shells = self.shells();
        *shells.last().unwrap_or(&0)
    }

    pub fn valency(&self) -> u8 {
        let valence = self.valence_electrons();
        let last_shell_capacity = match self.shells().len() {
            1 => 2,
            2 | 3 => 8,
            4 | 5 => 18,
            _ => 32,
        };

        if valence > last_shell_capacity / 2 {
            last_shell_capacity - valence
        } else {
            valence
        }
    }
}

pub const PERIODIC_TABLE: &[Element] = &[
    Element::new(1, "H", "Hydrogen"),
    Element::new(2, "He", "Helium"),
    Element::new(3, "Li", "Lithium"),
    Element::new(4, "Be", "Beryllium"),
    Element::new(5, "B", "Boron"),
    Element::new(6, "C", "Carbon"),
    Element::new(7, "N", "Nitrogen"),
    Element::new(8, "O", "Oxygen"),
    Element::new(9, "F", "Fluorine"),
    Element::new(10, "Ne", "Neon"),
    Element::new(11, "Na", "Sodium"),
    Element::new(12, "Mg", "Magnesium"),
    Element::new(13, "Al", "Aluminum"),
    Element::new(14, "Si", "Silicon"),
    Element::new(15, "P", "Phosphorus"),
    Element::new(16, "S", "Sulfur"),
    Element::new(17, "Cl", "Chlorine"),
    Element::new(18, "Ar", "Argon"),
];

pub fn get_by_symbol(symbol: &str) -> Option<Element> {
    PERIODIC_TABLE.iter().find(|e| e.symbol.eq_ignore_ascii_case(symbol)).copied()
}
