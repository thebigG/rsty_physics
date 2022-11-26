/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

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
fn divide(a: u32, b:u32) -> u32{
    if b == 0{
        panic!("Cannot divide by zero");
    }

    a/b
}

#[cfg(test)]
mod tests {
    use godot::engine::utilities::{sin, sqrt};
    // use godot::prelude::utilities::sin;
    use super::Rectangle;
    use super::add;
    use super::divide;
    use super::main_scene;

    #[test]
    fn larger_can_hold_smaller(){
        let larger = Rectangle{
            width: 8,
            height: 7,
        };

        let smaller = Rectangle{
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    /// These angles assume the "origin" angle is the one in question here.
    /// At the moment the godot functions are not working...
    #[test]
    fn sin_30_degrees(){
        let angle: f64  = 30.0;
        // assert_eq!(sin(angle.to_radians()), 0.5);
        //  assert_eq!(sqrt(angle.to_radians()), 0.5);

    }

}

#[gdextension]
unsafe impl ExtensionLibrary for RustyPhysics {}
