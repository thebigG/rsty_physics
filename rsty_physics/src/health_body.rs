use godot::engine::ICharacterBody2D;

use godot::engine::CharacterBody2D;
use godot::prelude::*;

// TODO:Implement when enum properties are supported
// enum State {
//     ALIVE = 1,
//     DEAD = 2,
// }

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct HealthBody2D {
    score: i64,
    #[base]
    base: Base<CharacterBody2D>,
    #[var(get = get_max_health)]
    MAX_HEALTH: real,
    #[var(get = get_zero_health)]
    ZERO_HEALTH: real,
    #[var(get = get_health)]
    health: real,
    #[var(get = get_damage_interval, set = set_damage_interval)]
    damage_interval: real,
    #[var(get = get_state)]
    state: i32,
}
#[godot_api]
impl HealthBody2D {
    #[func]
    fn game_over(&mut self) {
        println!("Game over!");
    }

    #[func]
    fn damage(&mut self) {
        if self.health <= self.ZERO_HEALTH {
            return;
        }
        self.health -= self.damage_interval;
        if self.health <= self.ZERO_HEALTH {
            self.state = 2;
        }
    }

    #[func]
    fn get_health(&mut self) -> real {
        self.health
    }

    #[func]
    fn get_damage_interval(&mut self) -> real {
        self.damage_interval
    }

    #[func]
    fn get_max_health(&mut self) -> real {
        self.MAX_HEALTH
    }

    #[func]
    fn get_zero_health(&mut self) -> real {
        self.ZERO_HEALTH
    }

    #[func]
    fn set_damage_interval(&mut self, new_interval: real) {
        self.damage_interval = new_interval;
    }

    #[func]
    fn get_state(&mut self) -> i32 {
        // match self.state {
        //     State::ALIVE => 1,
        //     State::DEAD => 0,
        // }
        self.state
    }
}

#[godot_api]
impl ICharacterBody2D for HealthBody2D {
    fn init(base: Base<Self::Base>) -> Self {
        HealthBody2D {
            base,
            score: 0,
            MAX_HEALTH: 1.0,
            ZERO_HEALTH: 0.0,
            health: 0.0,
            damage_interval: 0.0,
            state: 1,
        }
    }

    // fn ready(&mut self) {
    //     // Note: this is downcast during load() -- completely type-safe thanks to type inference!
    //     // If the resource does not exist or has an incompatible type, this panics.
    //     // There is also try_load() if you want to check whether loading succeeded.
    //     // self.mob_scene = load("res://Mob.tscn");
    //     // self.music = Some(self.base.get_node_as("Music"));
    //     // self.death_sound = Some(self.base.get_node_as("DeathSound"));
    //     // self.full_circle_sin();
    //     // self.draw_wave();
    //     // self.draw_circle();
    // }
}
