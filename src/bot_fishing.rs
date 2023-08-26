use std::{default::Default, thread, time};
use inputbot::MouseButton;
use crate::MinecraftMacro;

#[derive(Default)]
pub struct BoTFishing {}

impl MinecraftMacro for BoTFishing {
    fn macro_loop(&self) {
        MouseButton::LeftButton.press();
        MouseButton::LeftButton.release();
        thread::sleep(time::Duration::from_millis(30000));
    }

    fn name(&self) -> String {
        "BoT Fishing".to_string()
    }

    fn desc(&self) -> String {
        "Left clicks once every fifteen seconds.".to_string()
    }
}
