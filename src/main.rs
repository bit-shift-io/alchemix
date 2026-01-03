mod chemistry;
mod spell;

use std::io::{self, Write};
use chemistry::{Atom, get_by_symbol, bond_atoms};
use spell::map_molecule_to_spell;

fn main() {
    println!("--- Welcome to Alchemix ---");
    println!("Combine elements to discover spells.");
    println!("Enter symbols separated by spaces (e.g., 'H H O' for Water).");
    println!("Type 'exit' to quit.");

    loop {
        print!("\nElements > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        if input.is_empty() {
            continue;
        }

        let symbols: Vec<&str> = input.split_whitespace().collect();
        let mut atoms = Vec::new();
        let mut unknown_elements = Vec::new();

        for symbol in symbols {
            if let Some(element) = get_by_symbol(symbol) {
                atoms.push(Atom::new(element));
            } else {
                unknown_elements.push(symbol);
            }
        }

        if !unknown_elements.is_empty() {
            println!("Error: Unknown elements: {:?}", unknown_elements);
            continue;
        }

        match bond_atoms(atoms) {
            Some(molecule) => {
                println!("Success! Formed molecule: {}", molecule.name());
                let spell = map_molecule_to_spell(&molecule);
                println!("\n{}", spell.describe());
            }
            None => {
                println!("The elements refuse to bond. Check your valence shells!");
            }
        }
    }

    println!("Farewell, Alchemist.");
}
