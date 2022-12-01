/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use godot::engine::utilities::sin;
use godot::prelude::*;

struct RustyPhysics;

mod main_scene;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// Note that this function is "private" and our tests can still access it.
fn divide(a: u32, b: u32) -> u32 {
    if b == 0 {
        panic!("Cannot divide by zero");
    }

    a / b
}

//At the moment testing godot code the Rust way is not very trivial:https://github.com/godot-rust/gdextension/issues/36
// #[cfg(test)]
// mod tests {
//     use godot::engine::utilities::{sin, sqrt};
//     // use godot::prelude::utilities::sin;
//     use super::add;
//     use super::divide;
//     use super::main_scene;
//     use super::Rectangle;
//     use godot::prelude::*;

//     #[test]
//     fn sin_30_degrees() {
//         //  assert_eq!(sqrt(angle.to_radians()), 0.5);
//           let mut t = Trig{};
//           t.sin_30_degrees();
//     }

//     #[derive(GodotClass)]
//     #[class(base=Node)]
//     struct Trig{

//     }

//     #[test]
//     fn larger_can_hold_smaller() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };

//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };
//         assert!(larger.can_hold(&smaller));
//     }

//     #[cfg_attr(not(test), func)]
//     #[godot_api]
//     impl Trig{
//         #[func]
//         pub fn sin_30_degrees(&mut self) {
//             let angle: f64 = 30.0;
//             //  assert_eq!(sqrt(angle.to_radians()), 0.5);
//               assert_eq!(sin(angle.to_radians().round()), 0.5);
//         }
//     }

// }

#[gdextension]
unsafe impl ExtensionLibrary for RustyPhysics {}
