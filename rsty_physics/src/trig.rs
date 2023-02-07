use std::borrow::Borrow;
// use crate::hud::Hud;
// use crate::mob;
// use crate::player;
// use backtrace::Backtrace;
use godot::builtin::Vector2;
use godot::engine::node::InternalMode;
use godot::engine::packed_scene::GenEditState;
use godot::engine::utilities::{atan2, cos, deg_to_rad, pow, sin, sqrt};
use godot::engine::{Line2D, Marker2D, PathFollow2D, RigidBody2D, Timer};
use godot::prelude::*;
// use rand::Rng as _;
use godot::private::You_forgot_the_attribute__godot_api;
use std::f64::consts::PI;

use godot::builtin::VariantType::PackedVector2Array;
use godot::builtin::{Array, FromVariant, GodotString, ToVariant};
use godot::prelude::Variant;
use godot::sys::types::OpaquePackedVector2Array;

// use godot::sys::VariantType::Vector2;

// struct rsty_Vector2 {
//     x: f32,
//     y: f32
// }
//
// impl rsty_Vector2{
//     fn new() -> rsty_Vector2{
//         rsty_Vector2{
//             x: 0.0,
//             y: 0.0,
//         }
//     }
// }

///A particle made up of velocity and position
#[derive(GodotClass)]
#[class(base=Node)]
pub struct RstyVector2 {
    x: f32,
    y: f32,
    #[base]
    base: Base<Node>,
}
#[godot_api]
impl RstyVector2 {
    ///Stuck with set functions until properties are supported
    #[func]
    fn set_x(&mut self, value: f32) {
        self.x = value;
    }

    ///Stuck with set functions until properties are supported
    #[func]
    fn set_y(&mut self, value: f32) {
        self.y = value;
    }

    #[func]
    fn get_y(&mut self) -> f32{
        return self.y;
    }

    #[func]
    fn get_x(&mut self) -> f32{
        return self.x;
    }

    #[func]
    fn set_length(&mut self, length: f32) {
    let angle = self.get_angle();
    self.x = (cos(angle) as f32 * length) as f32;
    self.y = (sin(angle) as f32 * length) as f32;
  }
    #[func]
    fn get_length(&mut self) -> f64 {
    return sqrt(pow(self.x as f64, 2.0) + pow(self.y as f64, 2.0));
  }

    #[func]
    fn set_angle(&mut self, angle: f32) {
    let length = self.get_length();
    self.x = (cos(angle as f64) * length) as f32;
    self.y = (sin(angle as f64) * length) as f32;
  }

    #[func]
    fn get_angle(&mut self) -> f64 {
     atan2(self.y as f64, self.x as f64)
  }
    #[func]
    fn add(&mut self, mut v2: RstyVector2) -> Vector2 {
      Vector2{x: self.get_x() + v2.get_x(), y: self.get_y() + v2.get_y()}
  }
    #[func]
    fn substract(&mut self, mut v2: RstyVector2)-> Vector2 {
     Vector2{x: self.get_x() - v2.get_x(), y: self.get_y() - v2.get_y()}
  }

    #[func]
    fn multiply(&mut self, scalar: f32)-> Vector2 {
    return  Vector2{x: self.get_x() * scalar, y: self.get_y() * scalar};
  }

    fn divide(&mut self, scalar: f32)-> Vector2 {
    return Vector2{x: self.get_x() / scalar, y: self.get_y() / scalar};
  }
    #[func]
    fn get_godot_vector2(&mut self) -> Vector2{
        return Vector2{
            x: self.x,
            y: self.y,
        }
    }
    //At the moment gd-rust does not have support for godot properties.
    // So for now setters will have to do.
    // #[func]
    // fn init(&mut self, new_position: Vector2, new_speed: f32, direction: f32){
    //     self.position = new_position;
    //     let v = new_position.as_inner();
    // }
}
#[godot_api]
impl GodotExt for RstyVector2 {
    fn init(base: Base<Node>) -> Self {
        RstyVector2 {
            x: 0.0,
            y: 0.0,
            base,
        }
    }

    fn ready(&mut self) {
        // Note: this is downcast during load() -- completely type-safe thanks to type inference!
        // If the resource does not exist or has an incompatible type, this panics.
        // There is also try_load() if you want to check whether loading succeeded.
        // self.mob_scene = load("res://Mob.tscn");
        // self.music = Some(self.base.get_node_as("Music"));
        // self.death_sound = Some(self.base.get_node_as("DeathSound"));
        // self.draw_wave();
        // self.draw_circle();
    }
}

///A particle made up of velocity and position
#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Particle2D {
    position: Vector2,
    velocity: Vector2,
    #[base]
    base: Base<Node2D>,
}
#[godot_api]
impl Particle2D {
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

    ///At the moment gd-rust does not have support for godot properties.
    /// So for now setters will have to do.
    // #[func]
    // fn init(&mut self, new_position: Vector2, new_speed: f32, direction: f32){
    //     self.position = new_position;
    //     let v = new_position.as_inner();
    // }

    ///At the moment gd-rust does not have support for godot properties.
    /// So for now setters will have to do.
    #[func]
    fn set_position(&mut self, new_position: Vector2) {
        self.position = new_position;
    }

    #[func]
    fn set_velocity(&mut self, new_velocity: Vector2) {
        self.velocity = new_velocity;
    }
}

#[godot_api]
impl GodotExt for Particle2D {
    fn init(base: Base<Node2D>) -> Self {
        Particle2D {
            // mob_scene: PackedScene::new(),
            position: Default::default(),
            base,
            // music: None,
            // death_sound: None,
            velocity: Default::default(),
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

    #[func]
    fn get_sin_full_circle_2dvectors(
        &mut self,
        degrees_delta: i32,
        scale: i32,
        number_of_phases: i32,
    ) -> Array {
        let mut points = Array::from(&[Vector2::new(0.0, 100.0)]);
        let mut i: f32 = 0.0;
        // TODO:cleanup and remove all these casts
        let scale_f32: f32 = scale as f32;
        let phase = 2.0 * PI as f32;
        while i < (number_of_phases as f32 * phase) {
            let y: f32 = sin(i as f64) as f32;
            points.push(
                Vector2::new((i * scale_f32) as f32, (y as f32 * scale_f32) as f32).to_variant(),
            );
            i += deg_to_rad(degrees_delta as f64) as f32;
        }
        points
    }
    #[func]
    fn get_cos_full_circle_2dvectors(
        &mut self,
        degrees_delta: i32,
        scale: i32,
        number_of_phases: i32,
    ) -> Array {
        let mut points = Array::from(&[Vector2::new(0.0, 100.0)]);
        let mut i: f32 = 0.0;
        // TODO:cleanup and remove all these casts
        let scale_f32: f32 = scale as f32;
        let phase = 2.0 * PI as f32;
        while i < (number_of_phases as f32 * phase) {
            let y: f32 = cos(i as f64) as f32;
            points.push(
                Vector2::new((i * scale_f32) as f32, (y as f32 * scale_f32) as f32).to_variant(),
            );
            i += deg_to_rad(degrees_delta as f64) as f32;
        }
        points
    }

    // #[func]
    // fn add_two(&mut self, a: Variant) -> f32{
    //     let x: f32 = Variant::to(&a);
    //     println!("a:{}",a.get_type());
    //     return a + 2.0;
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
