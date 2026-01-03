mod chemistry;
mod spell;
mod enemy;
mod wave;

// No longer need std::io imports for input

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
    player_hp: i32,
    player_max_hp: i32,
    next_enemy_idx: usize,
}

impl GameState {
    fn new() -> Self {
        Self {
            atoms: Vec::new(),
            spells: Vec::new(),
            wave: Wave::generate(1),
            player_hp: 100,
            player_max_hp: 100,
            next_enemy_idx: 0,
        }
    }

    fn print_status(&self) {
        println!("\n--- GAME STATUS ---");
        println!("Player HP: {}/{}", self.player_hp, self.player_max_hp);
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
        for (i, enemy) in self.wave.enemies.iter().enumerate() {
            if enemy.is_alive() {
                println!("  [{}] {} - HP: {}/{}", i, enemy.name, enemy.hp, enemy.max_hp);
            } else {
                println!("  [{}] {} - DEFEATED", i, enemy.name);
            }
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

    fn perform_enemy_turn(&mut self) {
        let alive_enemies: Vec<usize> = self.wave.enemies.iter()
            .enumerate()
            .filter(|(_, e)| e.is_alive())
            .map(|(i, _)| i)
            .collect();

        if alive_enemies.is_empty() {
            return;
        }

        // Find the next alive enemy to take a turn
        let enemy_idx = if alive_enemies.contains(&self.next_enemy_idx) {
            self.next_enemy_idx
        } else {
            // Find the first alive enemy after the current next_enemy_idx
            *alive_enemies.iter()
                .find(|&&i| i > self.next_enemy_idx)
                .unwrap_or(&alive_enemies[0])
        };

        let enemy = &self.wave.enemies[enemy_idx];
        let damage = enemy.calculate_attack_damage();
        self.player_hp -= damage;

        println!("\n--- ENEMY TURN ---");
        println!("{} attacks you for {} damage!", enemy.name, damage);
        
        if self.player_hp <= 0 {
            self.player_hp = 0;
            println!("You have been defeated! Game Over.");
        } else {
            println!("Player HP: {}/{}", self.player_hp, self.player_max_hp);
        }

        // Update the index for the next turn
        // Find the index of the next alive enemy in the loop
        let current_pos = alive_enemies.iter().position(|&i| i == enemy_idx).unwrap();
        let next_pos = (current_pos + 1) % alive_enemies.len();
        self.next_enemy_idx = alive_enemies[next_pos];
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

    let mut rl = rustyline::DefaultEditor::new().expect("Failed to create rustyline editor");

    loop {
        let readline = rl.readline("\nAlchemix > ");
        let input = match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).expect("Failed to add history entry");
                line
            }
            Err(rustyline::error::ReadlineError::Interrupted) | Err(rustyline::error::ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        };

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
                println!("  cast <spell_idx> <idx/me>    - Cast a spell on an enemy or yourself");
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
                    println!("Usage: cast <spell_index> <enemy_index/me>");
                    continue;
                }

                let spell_idx: usize = match parts[1].parse() {
                    Ok(i) if i < state.spells.len() => i,
                    _ => { println!("Invalid spell index"); continue; }
                };

                let spell = &state.spells[spell_idx];

                if parts[2].to_lowercase() == "me" || parts[2].to_lowercase() == "self" {
                    println!("Casting {} on yourself!", spell.name);
                    
                    match spell.element_type {
                        crate::spell::molecule::SpellElement::Healing => {
                            let heal_amount = spell.power as i32;
                            let old_hp = state.player_hp;
                            state.player_hp = (state.player_hp + heal_amount).min(state.player_max_hp);
                            println!("You healed for {} HP! ({} -> {})", state.player_hp - old_hp, old_hp, state.player_hp);
                        }
                        _ => {
                            let damage = spell.power as i32;
                            state.player_hp -= damage;
                            println!("Ouch! You hit yourself for {} damage.", damage);
                        }
                    }
                } else {
                    let enemy_idx: usize = match parts[2].parse() {
                        Ok(i) if i < state.wave.enemies.len() => i,
                        _ => { println!("Invalid enemy index"); continue; }
                    };

                    let enemy = &mut state.wave.enemies[enemy_idx];
                    if !enemy.is_alive() {
                        println!("Enemy is already defeated!");
                        continue;
                    }

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
                        println!("Player collected the atoms from the enemy drop.");
                        state.atoms.extend(loot);
                    }
                }

                if state.wave.is_cleared() {
                    println!("\n--- ALL ENEMIES DEFEATED ---");
                    let next_wave_num = state.wave.number + 1;
                    println!("\n--- WAVE {} START ---", next_wave_num);
                    state.wave = Wave::generate(next_wave_num);
                    state.player_hp = state.player_max_hp;
                    state.next_enemy_idx = 0; // Reset turn cycle for new wave
                    state.print_status();
                } else {
                    state.perform_enemy_turn();
                }
                
                if state.player_hp <= 0 {
                    println!("\nBetter luck next time, Alchemist.");
                    break;
                }
            }
            _ => println!("Unknown command. Type 'help' for list of commands."),
        }
    }

    println!("Farewell, Alchemist.");
}
