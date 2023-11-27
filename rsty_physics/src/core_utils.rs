use godot::engine::{Animation, AnimationNode, AnimationNodeAnimation, IRefCounted};
use std::cell::Ref;

use godot::engine::RefCounted;
use godot::prelude::*;

enum State {
    ALIVE = 1,
    DEAD = 2,
}

// TODO:Implement when enum properties are supported
// enum HZ_MODE {
// 	UP = 1,
// 	DOWN = 2,
// };

#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct AnimationUtils {
    #[base]
    base: Base<RefCounted>,
    HZ_MODE: i32,
    max_health: real,
    zero_health: real,
    health: real,
    damage_interval: real,
    state: State,
    // animation: Animation ,
}
#[godot_api]
impl AnimationUtils {
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
    fn get_animation(&mut self) -> Gd<Animation> {
        let a = Animation::new();
        a
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
impl IRefCounted for AnimationUtils {
    fn init(base: Base<Self::Base>) -> Self {
        AnimationUtils {
            base,
            max_health: 1.0,
            zero_health: 0.0,
            health: 0.0,
            damage_interval: 0.0,
            state: State::ALIVE,
            // animation: Animation{ object_ptr: (), instance_id: () },
            HZ_MODE: 1,
        }
    }
}
