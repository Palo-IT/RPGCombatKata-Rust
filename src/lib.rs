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
    pub(crate) fn deal_damage(&self, defender: &mut Character, damage: i32) {
        defender.health -= damage;
        if defender.health <= 0 {
            defender.alive = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod character_creation {
        use super::*;

        #[test]
        fn created_character_has_by_default_1000_health_1_level_and_true_alive() {
            let character = Character::new();
            assert_eq!(character.health, 1000);
            assert_eq!(character.level, 1);
            assert_eq!(character.alive, true);
        }
    }

    mod character_can_deal_damage {
        use super::*;

        #[test]
        fn when_a_character_deals_100_damage_to_another_character_the_health_of_the_other_character_is_reduced_by_100() {
            let character = Character::new();
            let mut other_character = Character::new();
            character.deal_damage(&mut other_character, 100);
            assert_eq!(other_character.health, 900);
        }

        #[test]
        fn when_character_health_is_0_or_less_the_character_is_dead() {
            let character = Character::new();
            let mut other_character = Character::new();
            character.deal_damage(&mut other_character, 1000);
            assert_eq!(other_character.alive, false);
        }
    }
}
