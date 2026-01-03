mod chemistry;
mod spell;
mod enemy;
mod wave;

use std::io::{self, Write};
use chemistry::atom::Atom;
use chemistry::element::get_by_symbol;
use chemistry::reaction::bond_atoms;
use spell::mapping::map_molecule_to_spell;
use spell::molecule::Spell;

use wave::entity::Wave;

struct GameState {
    atoms: Vec<Atom>,
    spells: Vec<Spell>,
    wave: Wave,
    loot_pool: Vec<Atom>,
}

impl GameState {
    fn new() -> Self {
        Self {
            atoms: Vec::new(),
            spells: Vec::new(),
            wave: Wave::generate(1),
            loot_pool: Vec::new(),
        }
    }

    fn print_status(&self) {
        println!("\n--- GAME STATUS ---");
        println!("Wave: {}", self.wave.number);
        println!("Atoms: {}", self.format_atoms(&self.atoms));
        println!("Spells:");
        if self.spells.is_empty() {
            println!("  None");
        } else {
            for (i, spell) in self.spells.iter().enumerate() {
                println!("  [{}] {} ({:?}) - Power: {}", i, spell.name, spell.element_type, spell.power);
            }
        }
        
        println!("Enemies:");
        if self.wave.enemies.iter().all(|e| !e.is_alive()) {
            println!("  All enemies defeated! Type 'collect' to gather loot and proceed.");
        } else {
            for (i, enemy) in self.wave.enemies.iter().enumerate() {
                if enemy.is_alive() {
                    println!("  [{}] {} - HP: {}/{}", i, enemy.name, enemy.hp, enemy.max_hp);
                } else {
                    println!("  [{}] {} - DEFEATED", i, enemy.name);
                }
            }
        }

        if !self.loot_pool.is_empty() {
            println!("Loot on ground: {}", self.format_atoms(&self.loot_pool));
        }
    }

    fn format_atoms(&self, atoms: &[Atom]) -> String {
        if atoms.is_empty() {
            return "None".to_string();
        }
        let mut counts = std::collections::HashMap::new();
        for atom in atoms {
            *counts.entry(atom.element.symbol).or_insert(0) += 1;
        }
        counts.iter()
            .map(|(sym, count)| format!("{}({})", sym, count))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn main() {
    println!("--- Welcome to Alchemix ---");
    println!("Type 'help' for commands.");

    let mut state = GameState::new();

    // Give some starting atoms
    state.atoms.push(Atom::new(get_by_symbol("H").unwrap()));
    state.atoms.push(Atom::new(get_by_symbol("H").unwrap()));
    state.atoms.push(Atom::new(get_by_symbol("O").unwrap()));

    loop {
        print!("\nAlchemix > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0].to_lowercase().as_str() {
            "exit" | "quit" => break,
            "help" => {
                println!("Commands:");
                println!("  status                        - Show current game state");
                println!("  synth <sym1> <sym2> ...      - Synthesize a spell from atoms");
                println!("  cast <spell_idx> <enemy_idx> - Cast a spell on an enemy");
                println!("  collect                       - Pick up loot and start next wave");
                println!("  exit                          - Quit the game");
            }
            "status" => state.print_status(),
            "synth" => {
                if parts.len() < 2 {
                    println!("Usage: synth <symbol1> <symbol2> ...");
                    continue;
                }
                
                let mut requested_symbols = Vec::new();
                for &sym in &parts[1..] {
                    requested_symbols.push(sym);
                }

                // Check if player has the atoms
                let mut available_atoms = state.atoms.clone();
                let mut atoms_to_use = Vec::new();
                let mut missing = false;

                for &sym in &requested_symbols {
                    if let Some(pos) = available_atoms.iter().position(|a| a.element.symbol.eq_ignore_ascii_case(sym)) {
                        atoms_to_use.push(available_atoms.remove(pos));
                    } else {
                        println!("Error: Missing atom for {}", sym);
                        missing = true;
                        break;
                    }
                }

                if missing {
                    continue;
                }

                match bond_atoms(atoms_to_use) {
                    Some(molecule) => {
                        println!("Success! Formed molecule: {}", molecule.name());
                        let spell = map_molecule_to_spell(&molecule);
                        println!("New Spell Learned: {}", spell.name);
                        state.spells.push(spell);
                        state.atoms = available_atoms; // Consume used atoms
                    }
                    None => {
                        println!("The elements refuse to bond. Check your valence shells!");
                    }
                }
            }
            "cast" => {
                if parts.len() < 3 {
                    println!("Usage: cast <spell_index> <enemy_index>");
                    continue;
                }

                let spell_idx: usize = match parts[1].parse() {
                    Ok(i) if i < state.spells.len() => i,
                    _ => { println!("Invalid spell index"); continue; }
                };

                let enemy_idx: usize = match parts[2].parse() {
                    Ok(i) if i < state.wave.enemies.len() => i,
                    _ => { println!("Invalid enemy index"); continue; }
                };

                let enemy = &mut state.wave.enemies[enemy_idx];
                if !enemy.is_alive() {
                    println!("Enemy is already defeated!");
                    continue;
                }

                let spell = &state.spells[spell_idx];
                let (damage, is_crit) = enemy.take_damage(spell);
                
                println!("Casting {} on {}!", spell.name, enemy.name);
                if is_crit {
                    println!("CRITICAL HIT! It's super effective!");
                }
                println!("Dealt {} damage.", damage);

                if !enemy.is_alive() {
                    println!("{} has been defeated!", enemy.name);
                    let loot = enemy.drop_loot();
                    println!("Dropped: {}", state.format_atoms(&loot));
                    state.loot_pool.extend(loot);
                }
            }
            "collect" => {
                if !state.wave.is_cleared() {
                    println!("You must defeat all enemies before moving on!");
                    continue;
                }

                println!("Collected {} atoms from the ground.", state.loot_pool.len());
                state.atoms.extend(state.loot_pool.drain(..));
                
                let next_wave_num = state.wave.number + 1;
                println!("\n--- WAVE {} START ---", next_wave_num);
                state.wave = Wave::generate(next_wave_num);
                state.print_status();
            }
            _ => println!("Unknown command. Type 'help' for list of commands."),
        }
    }

    println!("Farewell, Alchemist.");
}
