use std::borrow::Borrow;
// use crate::hud::Hud;
// use crate::mob;
// use crate::player;
// use backtrace::Backtrace;
use godot::builtin::Vector2;
use godot::engine::node::InternalMode;
use godot::engine::packed_scene::GenEditState;
use godot::engine::utilities::{deg_to_rad, sin};
use godot::engine::{Line2D, Marker2D, PathFollow2D, RigidBody2D, Timer};
use godot::prelude::*;
// use rand::Rng as _;
use godot::private::You_forgot_the_attribute__godot_api;
use std::f64::consts::PI;

use godot::builtin::VariantType::PackedVector2Array;
use godot::builtin::{Array, FromVariant, GodotString, ToVariant};
use godot::sys::types::OpaquePackedVector2Array;
use godot::sys::VariantType::Object;
// use godot::sys::VariantType::Vector2;

#[derive(GodotClass)]
#[class(base=Line2D)]
pub struct SineWave2D {
    score: i64,
    #[base]
    base: Base<Line2D>,
}
#[godot_api]
impl SineWave2D {
    #[func]
    fn game_over(&mut self) {
        println!("Game over!");
    }

    #[func]
    pub fn new_game(&mut self) {
        // Obj::call();

        println!("New game");
        // let start_position = self.base.get_node_as::<Marker2D>("StartPosition");
        // let mut player = self.base.get_node_as::<player::Player>("Player");
        // let mut start_timer = self.base.get_node_as::<Timer>("StartTimer");
        //
        // self.score = 0;
        //
        // player.bind_mut().start(start_position.get_position());
        // start_timer.start(0.0);
        //
        // let mut hud = self.base.get_node_as::<Hud>("Hud");
        // let hud = hud.bind_mut();
        // hud.update_score(self.score);
        // hud.show_message("Get Ready".into());
        //
        // self.music().play(0.0);
    }

    #[func]
    fn test_sin(&mut self, angle: f64, answer: f64) {
        //Not sure if this is the "best" way to "round" numbers between 1.0 and 0.0, but it works.
        // do_some_work();

        assert_eq!(
            format!("{:.1}", (sin(angle.to_radians()) * 10.0)),
            format!("{:.1}", answer * 10.0)
        );
    }

    #[func]
    fn full_circle_sin(&mut self) {
        // Postive realm

        self.test_sin(30.0, 0.50);
        self.test_sin(60.0, 0.87);
        self.test_sin(90.0, 1.0);
        self.test_sin(120.0, 0.87);
        self.test_sin(150.0, 0.5);
        self.test_sin(180.0, f64::EPSILON);

        // Negative realm on the y-axis
        self.test_sin(210.0, -0.50);
        self.test_sin(240.0, -0.87);
        self.test_sin(270.0, -1.0);
        self.test_sin(300.0, -0.87);
        self.test_sin(330.0, -0.5);
        self.test_sin(360.0, -f64::EPSILON);
    }

    #[func]
    fn draw_wave(&mut self) -> Array {
        // let array = Array::from(&[1, 2]);
        let array = Array::from(&[Vector2::new(0.0, 100.0)]);
        println!("draw_wave1...");

        array
        //Update with new imports and other updates. Look at https://github.com/thebigG/gdextension/commit/6dbf0d7f06719538ac1ecb62125331bb50cd8659
        // let mut points = Array::new();
        // points;

        // points.append(Vector2::new(0.0, 100.0));
        // let v = Vector2Array::from(&points);

        // self.base.set_points(v)
    }
}

#[godot_api]
impl GodotExt for SineWave2D {
    fn init(base: Base<Line2D>) -> Self {
        SineWave2D {
            // mob_scene: PackedScene::new(),
            score: 0,
            base,
            // music: None,
            // death_sound: None,
        }
    }

    fn ready(&mut self) {
        // Note: this is downcast during load() -- completely type-safe thanks to type inference!
        // If the resource does not exist or has an incompatible type, this panics.
        // There is also try_load() if you want to check whether loading succeeded.
        // self.mob_scene = load("res://Mob.tscn");
        // self.music = Some(self.base.get_node_as("Music"));
        // self.death_sound = Some(self.base.get_node_as("DeathSound"));
        self.full_circle_sin();
        // self.draw_wave();
        // self.draw_circle();
    }
}

// Deriving GodotClass makes the class available to Godot
#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Main2D {
    score: i64,
    #[base]
    base: Base<Node2D>,
}
#[godot_api]
impl Main2D {
    #[func]
    fn game_over(&mut self) {
        println!("Game over!");
    }

    #[func]
    pub fn new_game(&mut self) {
        println!("New game");
        // let start_position = self.base.get_node_as::<Marker2D>("StartPosition");
        // let mut player = self.base.get_node_as::<player::Player>("Player");
        // let mut start_timer = self.base.get_node_as::<Timer>("StartTimer");
        //
        // self.score = 0;
        //
        // player.bind_mut().start(start_position.get_position());
        // start_timer.start(0.0);
        //
        // let mut hud = self.base.get_node_as::<Hud>("Hud");
        // let hud = hud.bind_mut();
        // hud.update_score(self.score);
        // hud.show_message("Get Ready".into());
        //
        // self.music().play(0.0);
    }

    #[func]
    fn test_sin(&mut self, angle: f64, answer: f64) {
        //Not sure if this is the "best" way to "round" numbers between 1.0 and 0.0, but it works.
        // do_some_work();

        assert_eq!(
            format!("{:.1}", (sin(angle.to_radians()) * 10.0)),
            format!("{:.1}", answer * 10.0)
        );
    }

    #[func]
    fn full_circle_sin(&mut self) {
        // Postive realm

        self.test_sin(30.0, 0.50);
        self.test_sin(60.0, 0.87);
        self.test_sin(90.0, 1.0);
        self.test_sin(120.0, 0.87);
        self.test_sin(150.0, 0.5);
        self.test_sin(180.0, f64::EPSILON);

        // Negative realm on the y-axis
        self.test_sin(210.0, -0.50);
        self.test_sin(240.0, -0.87);
        self.test_sin(270.0, -1.0);
        self.test_sin(300.0, -0.87);
        self.test_sin(330.0, -0.5);
        self.test_sin(360.0, -f64::EPSILON);
    }

    /// "draw_circle is a function in CanvasIem class. Don't want to override that."
    #[func]
    fn main_draw_circle(&mut self) {
        // let verticies = Vector2::new(10.0, 10.0);
        //
        // println!("{}", verticies);
        // let circle = Polygon2D;
    }

    // TODO:Finish impl
    // #[func]
    // fn get_sin_full_circle_2dvectors(&mut self, degrees_delta: f64, scale: f32, number_of_phases: f32) -> Array{
    //     let mut points = Array::from(&[Vector2::new(0.0, 100.0)]);;
    //     let mut i: f64 = 0.0;
    //     // TODO:cleanup and remove all these casts
    //     let tmp: f32 = 2.0;
    //     let tmp_pi: f32 = PI as f32;
    //     println!("get_sin_full_circle_2dvectors1:{}", (number_of_phases * (tmp * tmp_pi)));
    //     while i < (number_of_phases * (tmp * tmp_pi)) as f64 {
    //         let x: f32 = sin(i) as f32;
    //         println!("get_sin_full_circle_2dvectors2");
    //         // TODO:Not sure why the sin/cos functions are using f64 and the vectors are using f32
    //         println!("{}",i);
    // 	    points.push(Vector2::new((i as f32 * scale as f32) as f32, (x as f32 * scale as f32) as f32).to_variant());
    //         i += deg_to_rad(degrees_delta);
    //     }
    //     println!("get_sin_full_circle_2dvectors3");
    //     points
    // }
}

#[godot_api]
impl GodotExt for Main2D {
    fn init(base: Base<Node2D>) -> Self {
        Main2D {
            // mob_scene: PackedScene::new(),
            score: 0,
            base,
            // music: None,
            // death_sound: None,
        }
    }

    fn ready(&mut self) {
        // Note: this is downcast during load() -- completely type-safe thanks to type inference!
        // If the resource does not exist or has an incompatible type, this panics.
        // There is also try_load() if you want to check whether loading succeeded.
        // self.mob_scene = load("res://Mob.tscn");
        // self.music = Some(self.base.get_node_as("Music"));
        // self.death_sound = Some(self.base.get_node_as("DeathSound"));
        println!("Main...");
        self.full_circle_sin();
        self.main_draw_circle();
    }
}
