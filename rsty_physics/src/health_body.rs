use godot::engine::ICharacterBody2D;

use godot::engine::CharacterBody2D;
use godot::prelude::*;

enum State {
    ALIVE = 1,
    DEAD = 2,
}

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct HealthBody2D {
    score: i64,
    #[base]
    base: Base<CharacterBody2D>,
    max_health: real,
    zero_health: real,
    health: real,
    damage_interval: real,
    state: State,
}
#[godot_api]
impl HealthBody2D {
    #[func]
    fn game_over(&mut self) {
        println!("Game over!");
    }

    #[func]
    fn damage(&mut self) {
        if self.health <= self.zero_health {
            return;
        }
        self.health -= self.damage_interval;
        if self.health <= self.zero_health {
            self.state = State::DEAD;
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
        self.max_health
    }

    #[func]
    fn get_zero_health(&mut self) -> real {
        self.zero_health
    }

    #[func]
    fn set_damage_interval(&mut self, new_interval: real) {
        self.damage_interval = new_interval;
    }

    #[func]
    fn get_state(&mut self) -> i32 {
        match self.state {
            State::ALIVE => 1,
            State::DEAD => 0,
        }
    }
}

#[godot_api]
impl ICharacterBody2D for HealthBody2D {
    fn init(base: Base<Self::Base>) -> Self {
        HealthBody2D {
            base,
            score: 0,
            max_health: 1.0,
            zero_health: 0.0,
            health: 0.0,
            damage_interval: 0.0,
            state: State::ALIVE,
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
