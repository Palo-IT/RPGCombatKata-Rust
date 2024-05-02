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

    pub(crate) fn heal(&mut self, health: i32) {
        if !self.alive {
            return;
        }
        self.health = (self.health + health).min(1000);
    }

    pub(crate) fn deal_damage(&self, defender: &mut Character, damage: i32) {
        let damage = if defender.level >= self.level + 5 {
            damage / 2
        } else {
            damage
        };
        defender.health -= damage;
        if defender.health <= 0 {
            defender.alive = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Character {
        Character::new()
    }

    mod character_creation {
        use super::*;

        #[test]
        fn created_character_has_by_default_1000_health_1_level_and_true_alive() {
            let character = setup();
            assert_eq!(character.health, 1000);
            assert_eq!(character.level, 1);
            assert_eq!(character.alive, true);
        }
    }

    mod character_can_deal_damage {
        use super::*;

        #[test]
        fn when_a_character_deals_100_damage_to_another_character_the_health_of_the_other_character_is_reduced_by_100() {
            let character = setup();
            let mut other_character = setup();
            character.deal_damage(&mut other_character, 100);
            assert_eq!(other_character.health, 900);
        }

        #[test]
        fn when_character_health_is_0_or_less_the_character_is_dead() {
            let character = setup();
            let mut other_character = setup();
            character.deal_damage(&mut other_character, 1000);
            assert_eq!(other_character.alive, false);
        }

        #[test]
        fn when_dealing_damage_to_target_with_level_5_or_more_above_the_attacker_the_damage_is_reduced_by_50_percent() {
            let attacker = setup();
            let mut target = setup();
            target.level = attacker.level + 5;
            attacker.deal_damage(&mut target, 100);
            assert_eq!(target.health, 950);
        }

        //region This test is not needed because a character cannot deal damage to itself due to the borrow checker
        //  this code is kept for further discussion
        // #[test]
        // fn a_character_cannot_deal_damage_to_itself() {
        //     let mut character = setup();
        //     let mut other_character = setup();
        //     character.deal_damage(&mut other_character, 100);
        //     character.deal_damage(&mut character, 100);
        //     assert_eq!(character.health, 1000);
        // }
        //endregion
    }

    mod character_can_heal_a_character {
        use super::*;

        #[test]
        fn when_a_character_heals_100_health_to_another_character_the_health_of_the_other_character_is_increased_by_100() {
            let character = setup();
            let mut other_character = setup();
            character.deal_damage(&mut other_character, 200);
            other_character.heal(100);
            assert_eq!(other_character.health, 900);
        }

        #[test]
        fn dead_character_cannot_be_healed() {
            let character = setup();
            let mut other_character = setup();
            character.deal_damage(&mut other_character, 1000);
            other_character.heal(100);
            assert_eq!(other_character.health, 0);
        }

        #[test]
        fn healing_cannot_increase_health_above_1000() {
            let character = setup();
            let mut other_character = setup();
            character.deal_damage(&mut other_character, 10);
            other_character.heal(100);
            assert_eq!(other_character.health, 1000);
        }

        //region This will become useless as the `healed` parameter is removed from the heal function
        // #[test]
        // fn a_character_can_only_heal_itself() {
        //     let character = setup();
        //     let mut other_character = setup();
        //     character.deal_damage(&mut other_character, 200);
        //     other_character.heal(&mut other_character, 100);
        //     assert_eq!(other_character.health, 900);
        // }
        //endregion
    }
}
