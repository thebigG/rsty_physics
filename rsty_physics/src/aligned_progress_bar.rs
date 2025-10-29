use godot::classes::IProgressBar;

use godot::classes::ProgressBar;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=ProgressBar)]
pub struct AlignedProgressBar {
    base: Base<ProgressBar>,
    #[var(get = get_alignment, set = set_alignment)]
    alignment: i32,
}
#[godot_api]
impl AlignedProgressBar {
    #[func]
    fn set_alignment(&mut self, new_alignment: i32) {
        self.alignment = new_alignment;
    }
    #[func]
    fn get_alignment(&mut self) -> i32 {
        self.alignment
    }
}

// TODO:Implement when enum properties are supported
// enum HorizontalAlignment {
// 	HORIZONTAL_ALIGNMENT_LEFT = 0,
// 	HORIZONTAL_ALIGNMENT_CENTER = 1,
// 	HORIZONTAL_ALIGNMENT_RIGHT = 2,
// 	HORIZONTAL_ALIGNMENT_FILL = 3,
// };

#[godot_api]
impl IProgressBar for AlignedProgressBar {
    fn init(base: Base<Self::Base>) -> Self {
        AlignedProgressBar { base, alignment: 0 }
    }
}
