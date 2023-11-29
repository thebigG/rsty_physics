use godot::engine::{animation, Animation, AnimationNode, AnimationNodeAnimation, IRefCounted};
use std::borrow::Borrow;
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

pub struct AnimationData {
    animation: Gd<Animation>,
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct AnimationUtils {
    #[base]
    base: Base<Node>,
    HZ_MODE: i32,
    max_health: real,
    zero_health: real,
    health: real,
    damage_interval: real,
    state: State,
    animation: Gd<Animation>,
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
    fn get_animation(
        &mut self,
        node_path: NodePath,
        text: String,
        delimiter: String,
    ) -> Gd<Animation> {
        // Not sure if animation as a member of the struct makes much sense...
        // let  a = Animation::new();
        // a.get_length();

        let track_index = self.animation.add_track(animation::TrackType::TYPE_VALUE);
        let tokens = text.split(&delimiter);
        let mut current_text = String::new();
        let mut current_transition = 0.0;
        for token in tokens {
            current_text = current_text.clone() + token + &delimiter.clone();
            self.animation.track_insert_key(
                track_index,
                current_transition,
                current_text.to_variant(),
            );
        }
        self.animation.clone()
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

// #[godot_api]
// impl IRefCounted for AnimationUtils {
//     fn init(base: Base<Self::Base>) -> Self {
//         AnimationUtils {
//             base,
//             max_health: 1.0,
//             zero_health: 0.0,
//             health: 0.0,
//             damage_interval: 0.0,
//             state: State::ALIVE,
//             animation: AnimationData{ animation: Animation::new() },
//             HZ_MODE: 1,
//         }
//     }
// }

#[godot_api]
impl INode for AnimationUtils {
    fn init(base: Base<Self::Base>) -> Self {
        AnimationUtils {
            base,
            max_health: 1.0,
            zero_health: 0.0,
            health: 0.0,
            damage_interval: 0.0,
            state: State::ALIVE,
            animation: Animation::new(),
            HZ_MODE: 1,
        }
    }
}
