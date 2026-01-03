use crate::chemistry::reaction::MoleculeCandidate;
use crate::spell::molecule::{Spell, SpellElement};

pub fn map_molecule_to_spell(molecule: &MoleculeCandidate) -> Spell {
    let formula = molecule.name();
    
    // Hardcoded recipes for common molecules
    match formula.as_str() {
        "H2O" => Spell {
            name: "Hydrostream".to_string(),
            power: 10,
            element_type: SpellElement::Healing,
            effects: vec!["Drenching".to_string(), "Minor Mending".to_string()],
        },
        "CO2" => Spell {
            name: "Choke Cloud".to_string(),
            power: 15,
            element_type: SpellElement::Gaseous,
            effects: vec!["Oxygen Depletion".to_string(), "Extinguish Fire".to_string()],
        },
        "O2" => Spell {
            name: "Flame Fuel".to_string(),
            power: 20,
            element_type: SpellElement::Fire,
            effects: vec!["Combustion Support".to_string()],
        },
        "NaCl" => Spell {
            name: "Salt Shard".to_string(),
            power: 12,
            element_type: SpellElement::Physical,
            effects: vec!["Crystallization".to_string(), "Desiccation".to_string()],
        },
        "H2" => Spell {
            name: "Hydrogen Pop".to_string(),
            power: 25,
            element_type: SpellElement::Fire,
            effects: vec!["Explosive".to_string()],
        },
        "CH4" => Spell {
            name: "Methane Blast".to_string(),
            power: 30,
            element_type: SpellElement::Fire,
            effects: vec!["Foul Smelling".to_string(), "Highly Flammable".to_string()],
        },
        _ => generate_generic_spell(molecule),
    }
}

fn generate_generic_spell(molecule: &MoleculeCandidate) -> Spell {
    let formula = molecule.name();
    let power = (molecule.atoms.len() * 5) as u32;
    
    // Very basic heuristic for generic elements
    let has_flammable = molecule.atoms.iter().any(|a| a.element.symbol == "H" || a.element.symbol == "C" || a.element.symbol == "S");
    let has_oxidizer = molecule.atoms.iter().any(|a| a.element.symbol == "O" || a.element.symbol == "F" || a.element.symbol == "Cl");
    
    let element_type = if has_flammable && has_oxidizer {
        SpellElement::Fire
    } else if has_oxidizer {
        SpellElement::Poison
    } else {
        SpellElement::Physical
    };

    Spell {
        name: format!("Molecular Bond: {}", formula),
        power,
        element_type,
        effects: vec!["Unstable Compound".to_string()],
    }
}
