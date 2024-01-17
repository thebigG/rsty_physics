use godot::engine::{
    animation, Animation, AnimationNode, AnimationNodeAnimation, Curve2D, IRefCounted,
};
use std::borrow::Borrow;
use std::cell::Ref;
use std::f32::consts::PI;

use godot::engine::RefCounted;
use godot::prelude::*;
// use godot::sys::VariantType::Vector2;
use godot::builtin::Vector2;
use godot::engine::utilities::{deg_to_rad, sin};

use crate::trig::Main2D;

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
        self.animation.track_set_path(track_index, node_path);
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

    ///
    /// @brief AnimationUtils::h_line_pattern Draws a horizontal line on the path
    /// that starts at origin that is length long.
    /// @param path
    /// @param origin
    /// @param length
    /// @return The last point that was added to curve.
    #[func]
    fn h_line_pattern(&mut self, mut path: Gd<Curve2D>, origin: Vector2, length: f32) -> Vector2 {
        let target = Vector2::new(origin.x + length, origin.y);
        // path.add_point(origin, Vector2::new(0.0,0.0) , Vector2::new(0.0,0.0));
        path.add_point(origin);
        path.add_point(origin);
        target
    }
    ///
    /// @brief AnimationUtils::h_line_pattern Draws a horizontal line on the path
    /// that starts at origin that is length long.
    /// @param path
    /// @param origin
    /// @param length
    /// @return The last point that was added to curve.
    #[func]
    fn v_line_pattern(&mut self, mut path: Gd<Curve2D>, origin: Vector2, length: f32) -> Vector2 {
        let target = Vector2::new(origin.x, origin.y + length);
        // path.add_point(origin, Vector2::new(0.0,0.0) , Vector2::new(0.0,0.0));
        path.add_point(origin);
        path.add_point(origin);
        target
    }

    ///
    /// @brief AnimationUtils::h_line_pattern Draws a horizontal line on the path
    /// that starts at origin that is length long.
    /// @param path
    /// @param origin
    /// @param length
    /// @return The last point that was added to curve.
    #[func]
    fn hz_line_pattern(
        &mut self,
        mut path: Gd<Curve2D>,
        origin: Vector2,
        length: f32,
        mode: i32,
    ) -> Vector2 {
        let mut target = Vector2::new(origin.x + length, origin.y + length);
        // path.add_point(origin, Vector2::new(0.0,0.0) , Vector2::new(0.0,0.0));

        match mode {
            2 => {
                target = Vector2::new(origin.x + length, origin.y + length);
            }
            1 => {
                target = Vector2::new(origin.x + length, origin.y - length);
            }

            m => {
                // TODO: Need to change mode to enums instead of i32
                // Maybe a better way of handling this is returning a Result...
                panic!("Unsupported mode:{m}")
            }
        }
        path.add_point(origin);
        path.add_point(target);
        target
    }

    ///
    /// @brief AnimationUtils::h_line_pattern Draws a horizontal line on the path
    /// that starts at origin that is length long.
    /// @param path
    /// @param origin
    /// @param length
    /// @return The last point that was added to curve.
    #[func]
    fn rectangle_pattern(
        &mut self,
        path: Gd<Curve2D>,
        origin: Vector2,
        width: f32,
        height: f32,
    ) -> Vector2 {
        let top = self.h_line_pattern(path.clone(), origin, width);
        let right = self.v_line_pattern(path.clone(), top, height);
        let bottom = self.h_line_pattern(path.clone(), right, -width);
        let rect = self.h_line_pattern(path.clone(), right, -height);

        rect
    }

    ///
    /// @brief AnimationUtils::h_line_pattern Draws a horizontal line on the path
    /// that starts at origin that is length long.
    /// @param path
    /// @param origin
    /// @param length
    /// @return The last point that was added to curve.
    #[func]
    fn zig_zag_pattern(
        &mut self,
        path: Gd<Curve2D>,
        origin: Vector2,
        length: f32,
        zigs: i32,
    ) -> Vector2 {
        let mut last_origin = origin;
        for zig in 0..zigs {
            if (zig % 2 == 0) {
                last_origin = self.hz_line_pattern(path.clone(), last_origin, length, 2);
            } else {
                last_origin = self.hz_line_pattern(path.clone(), last_origin, length, 1);
            }
        }

        return last_origin;
    }

    ///
    /// @brief AnimationUtils::h_line_pattern Draws a horizontal line on the path
    /// that starts at origin that is length long.
    /// @param path
    /// @param origin
    /// @param length
    /// @return The last point that was added to curve.
    #[func]
    fn circle_pattern(
        &mut self,
        mut path: Gd<Curve2D>,
        origin: Vector2,
        degrees_delta: i32,
        scale: i32,
        number_of_phases: i32,
    ) -> () {
        let mut last_origin = origin;

        let mut points = array![Vector2::new(origin.x, origin.y).to_variant()];
        let mut i: f32 = 0.0;
        // TODO:cleanup and remove all these casts
        let scale_f32: f32 = scale as f32;
        let phase = 2.0 * PI as f32;
        while i < (number_of_phases as f32 * phase) {
            let y: f32 = sin(i as f64) as f32;
            points.push(
                Vector2::new((i * scale_f32) as f32, (y as f32 * scale_f32) as f32).to_variant(),
            );
            path.add_point(Vector2::new(
                (i * scale_f32) as f32,
                (y as f32 * scale_f32) as f32,
            ));
            i += deg_to_rad(degrees_delta as f64) as f32;
        }

        // last_origin
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
