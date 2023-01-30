use std::{default::Default, thread, time};
use inputbot::MouseButton;
use crate::MinecraftMacro;

#[derive(Default)]
pub struct RightClick {}

impl MinecraftMacro for RightClick {
    fn macro_loop(&self) {
        MouseButton::RightButton.press();
        MouseButton::RightButton.release();
        thread::sleep(time::Duration::from_millis(50));
    }

    fn name(&self) -> String {
        "Right Click".to_string()
    }

    fn desc(&self) -> String {
        "Right clicks 20 times per second.".to_string()
    }
}
