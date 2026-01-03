use crate::spell::molecule::{Spell, SpellElement};
use crate::chemistry::atom::Atom;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemyType {
    FireSpirit,
    FrostGolem,
    Slime,
    ThunderBird,
    EarthElemental,
}

impl EnemyType {
    pub fn vulnerability(&self) -> SpellElement {
        match self {
            EnemyType::FireSpirit => SpellElement::Frost,
            EnemyType::FrostGolem => SpellElement::Fire,
            EnemyType::Slime => SpellElement::Poison,
            EnemyType::ThunderBird => SpellElement::Physical,
            EnemyType::EarthElemental => SpellElement::Electric,
        }
    }

    pub fn element_affinity(&self) -> &'static str {
        match self {
            EnemyType::FireSpirit => "H", // Hydrogen (flammable)
            EnemyType::FrostGolem => "O", // Oxygen (related to H2O)
            EnemyType::Slime => "S",      // Sulfur
            EnemyType::ThunderBird => "Cl", // Chlorine
            EnemyType::EarthElemental => "C", // Carbon
        }
    }
}

pub struct Enemy {
    pub name: String,
    pub enemy_type: EnemyType,
    pub hp: i32,
    pub max_hp: i32,
}

impl Enemy {
    pub fn new(enemy_type: EnemyType, level: i32) -> Self {
        let max_hp = 20 + level * 10;
        let name = format!("{:?} (Lv. {})", enemy_type, level);
        Self {
            name,
            enemy_type,
            hp: max_hp,
            max_hp,
        }
    }

    pub fn take_damage(&mut self, spell: &Spell) -> (i32, bool) {
        let is_vulnerable = spell.element_type == self.enemy_type.vulnerability();
        let multiplier = if is_vulnerable { 2.5 } else { 1.0 };
        let damage = (spell.power as f32 * multiplier) as i32;
        self.hp -= damage;
        (damage, is_vulnerable)
    }

    pub fn drop_loot(&self) -> Vec<Atom> {
        let mut rng = rand::rng();
        let symbol = self.enemy_type.element_affinity();
        let element = crate::chemistry::element::get_by_symbol(symbol).unwrap();
        
        let count = rng.random_range(1..=3);
        (0..count).map(|_| Atom::new(element)).collect()
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub fn calculate_attack_damage(&self) -> i32 {
        let mut rng = rand::rng();
        let _base_damage = self.name.len() as i32 / 2; // Simple base damage related to name
        let _level_scaling = (self.hp as f32 / self.max_hp as f32 * 5.0) as i32; // Scaling by HP percentage
        
        // Let's use a simpler formula based on level as planned
        let level = (self.max_hp - 20) / 10;
        level * 3 + rng.random_range(1..5)
    }
}
