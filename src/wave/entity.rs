use crate::enemy::entity::{Enemy, EnemyType};
use rand::Rng;

pub struct Wave {
    pub number: i32,
    pub enemies: Vec<Enemy>,
}

impl Wave {
    pub fn generate(number: i32) -> Self {
        let mut rng = rand::rng();
        let mut enemies = Vec::new();
        
        // More enemies as waves progress
        let count = 1 + (number / 2) + rng.random_range(0..=1);
        
        for _i in 0..count {
            let enemy_type = match rng.random_range(0..5) {
                0 => EnemyType::FireSpirit,
                1 => EnemyType::FrostGolem,
                2 => EnemyType::Slime,
                3 => EnemyType::ThunderBird,
                _ => EnemyType::EarthElemental,
            };
            
            // Stronger enemies as waves progress, with some randomness
            let level = number + rng.random_range(-1..=1).max(0);
            enemies.push(Enemy::new(enemy_type, level));
        }
        
        Self { number, enemies }
    }

    pub fn is_cleared(&self) -> bool {
        self.enemies.iter().all(|e| !e.is_alive())
    }
}
