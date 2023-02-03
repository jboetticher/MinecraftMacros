use std::{default::Default, thread, time};
use inputbot::MouseButton;
use crate::MinecraftMacro;

#[derive(Default)]
pub struct LeftClick {}

impl MinecraftMacro for LeftClick {
    fn macro_loop(&self) {
        MouseButton::LeftButton.press();
        MouseButton::LeftButton.release();
        thread::sleep(time::Duration::from_millis(50));
    }

    fn name(&self) -> String {
        "Left Click".to_string()
    }

    fn desc(&self) -> String {
        "Left clicks 20 times per second.".to_string()
    }
}
