#[derive(Debug, Clone)]
pub struct Spell {
    pub name: String,
    pub power: u32,
    pub element_type: SpellElement,
    pub effects: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellElement {
    Fire,
    Frost,
    Poison,
    Healing,
    Electric,
    Physical,
    Gaseous,
}

impl Spell {
    pub fn describe(&self) -> String {
        format!(
            "Spell: {}\nType: {:?}\nPower: {}\nEffects: {}",
            self.name,
            self.element_type,
            self.power,
            self.effects.join(", ")
        )
    }
}
