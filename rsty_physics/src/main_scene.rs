// use crate::hud::Hud;
// use crate::mob;
// use crate::player;
// use backtrace::Backtrace;
use godot::engine::node::InternalMode;
use godot::engine::packed_scene::GenEditState;
use godot::engine::utilities::sin;
use godot::engine::{Marker2D, PathFollow2D, RigidBody2D, Timer};
use godot::prelude::*;
// use rand::Rng as _;
use godot::private::You_forgot_the_attribute__godot_api;
use std::backtrace::{self, Backtrace};
use std::f64::consts::PI;

// Deriving GodotClass makes the class available to Godot
#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    score: i64,
    #[base]
    base: Base<Node>,
}
#[godot_api]
impl Main {
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
            format!("{:.1}", answer * 10.0),
            "{}",
            Backtrace::capture()
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
}

#[godot_api]
impl GodotExt for Main {
    fn init(base: Base<Node>) -> Self {
        Main {
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
    }
}
