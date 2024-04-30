struct Character {
    health: i32,
    level: i32,
    alive: bool,
}

impl Character {
    fn new() -> Self {
        Self {
            health: 1000,
            level: 1,
            alive: true,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn created_character_has_by_default_1000_health_1_level_and_true_alive() {
        let character = Character::new();
        assert_eq!(character.health, 1000);
        assert_eq!(character.level, 1);
        assert_eq!(character.alive, true);
    }
}
