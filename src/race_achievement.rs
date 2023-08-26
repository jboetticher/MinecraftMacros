use std::{default::Default, thread, time};
use inputbot::{MouseCursor, MouseButton, KeybdKey, KeySequence};
use crate::MinecraftMacro;

#[derive(Default)]
pub struct RaceAchievement {}

// This should only work on a 1080x1920 screen
impl MinecraftMacro for RaceAchievement {
    fn macro_loop(&self) {
        // 1. Move to the settings button & click
        MouseCursor::move_abs(1600, 42);
        thread::sleep(time::Duration::from_millis(500));
        MouseButton::LeftButton.press();
        MouseButton::LeftButton.release();
        thread::sleep(time::Duration::from_millis(500));

        // 2. Select the restart button
        MouseCursor::move_abs(1077, 853);
        thread::sleep(time::Duration::from_millis(500));
        MouseButton::LeftButton.press();
        MouseButton::LeftButton.release();
        thread::sleep(time::Duration::from_millis(2000));
    }

    fn name(&self) -> String {
        "Bloons Race Achievement".to_string()
    }

    fn desc(&self) -> String {
        "Restarts the race repeatedly.".to_string()
    }
}