use std::{default::Default, thread, time};
use inputbot::{MouseCursor, MouseButton, KeybdKey, KeySequence};
use crate::MinecraftMacro;

#[derive(Default)]
pub struct Craft {}

// This should only work on a 1080x1920 screen
impl MinecraftMacro for Craft {
    fn macro_loop(&self) {
        println!("Starting craft");
        // 1. Move to the search section
        MouseCursor::move_abs(100, 100);
        thread::sleep(time::Duration::from_millis(100));
        println!("Step 1 finished");

        // 2. Input the text
        KeySequence("Melon").send();
        thread::sleep(time::Duration::from_millis(100));
        println!("Step 2 finished");

        // 3. Select the first thing that pops up
        MouseCursor::move_abs(100, 50);
        thread::sleep(time::Duration::from_millis(100));
        MouseButton::LeftButton.press();
        MouseButton::LeftButton.release();
        thread::sleep(time::Duration::from_millis(100));
        println!("Step 3 finished");

        // 4. Move to the crafting section
        MouseCursor::move_abs(300, 50);
        thread::sleep(time::Duration::from_millis(100));
        println!("Step 4 finished");

        // 5. Press Q 64 times
        for _ in 0..64 {
            KeybdKey::QKey.press();
            KeybdKey::QKey.release();
            thread::sleep(time::Duration::from_millis(20));
        }
        println!("Step 5 finished");

    }

    fn name(&self) -> String {
        "Craft a Melon".to_string()
    }

    fn desc(&self) -> String {
        "Rapidly crafts a melon.".to_string()
    }
}
