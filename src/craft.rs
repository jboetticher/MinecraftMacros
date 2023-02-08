use std::{default::Default, thread, time};
use inputbot::{MouseCursor, MouseButton, KeybdKey, KeySequence};
use crate::MinecraftMacro;

#[derive(Default)]
pub struct Craft {}

// This should only work on a 1080x1920 screen
impl MinecraftMacro for Craft {
    fn macro_loop(&self) {
        // 1. Move to the search section & click
        MouseCursor::move_abs(430, 300);
        thread::sleep(time::Duration::from_millis(50));
        MouseButton::LeftButton.press();
        MouseButton::LeftButton.release();
        thread::sleep(time::Duration::from_millis(50));

        // 2. Select the first thing that pops up in crafting menu
        MouseCursor::move_abs(415, 400);
        thread::sleep(time::Duration::from_millis(50));
        KeybdKey::LShiftKey.press();
        MouseButton::LeftButton.press();
        MouseButton::LeftButton.release();
        KeybdKey::LShiftKey.release();
        thread::sleep(time::Duration::from_millis(50));

        // 3. Move elsewhere to click & ensure that the Q Key is ok
        MouseCursor::move_abs(1450, 300);
        thread::sleep(time::Duration::from_millis(50));
        MouseButton::LeftButton.press();
        MouseButton::LeftButton.release();
        thread::sleep(time::Duration::from_millis(50));

        // 3. Move to the crafting section
        MouseCursor::move_abs(1450, 400);
        thread::sleep(time::Duration::from_millis(50));

        // 5. Press Q 64 times
        for _ in 0..64 {
            KeybdKey::QKey.press();
            KeybdKey::QKey.release();
            thread::sleep(time::Duration::from_millis(30));
        }
    }

    fn name(&self) -> String {
        "Craft a Melon".to_string()
    }

    fn desc(&self) -> String {
        "Rapidly crafts and drops.".to_string()
    }
}
