use std::{default::Default, thread, time};
use inputbot::{MouseCursor, MouseButton, KeybdKey, KeySequence};
use crate::MinecraftMacro;

#[derive(Default)]
pub struct TempRace {}

// This should only work on a 1080x1920 screen
impl MinecraftMacro for TempRace {
    fn macro_loop(&self) {
        let ace = spawn_monkey(733, 316, Monkeys::Ace);




        // 2. Select the first thing that pops up in crafting menu
        // MouseCursor::move_abs(415, 400);
        // thread::sleep(time::Duration::from_millis(50));
        // KeybdKey::LShiftKey.press();
        // MouseButton::LeftButton.press();
        // MouseButton::LeftButton.release();
        // KeybdKey::LShiftKey.release();
        // thread::sleep(time::Duration::from_millis(50));

        // // 3. Move elsewhere to click & ensure that the Q Key is ok
        // MouseCursor::move_abs(1450, 300);
        // thread::sleep(time::Duration::from_millis(50));
        // MouseButton::LeftButton.press();
        // MouseButton::LeftButton.release();
        // thread::sleep(time::Duration::from_millis(50));

        // // 3. Move to the crafting section
        // MouseCursor::move_abs(1450, 400);
        // thread::sleep(time::Duration::from_millis(50));
    }

    fn name(&self) -> String {
        "Race Macro".to_string()
    }

    fn desc(&self) -> String {
        "Wins the Fast Bloons, Slooooow MOABs race.".to_string()
    }
}

struct MonkeyInstance {
    x: i32,
    y: i32
}

fn spawn_monkey(x: i32, y: i32, m: Monkeys) -> MonkeyInstance {
    MouseCursor::move_abs(x, y);
    thread::sleep(time::Duration::from_millis(50));

    match m {
        Monkeys::Ace => {
            KeybdKey::VKey.press();
            KeybdKey::VKey.release();
        },
        Monkeys::TackShooter => {
            KeybdKey::RKey.press();
            KeybdKey::RKey.release();
        },
        default => {
            println!("Monkey could not be spawned!");
        }
    }

    MouseButton::LeftButton.press();
    MouseButton::LeftButton.release();

    MonkeyInstance { x, y }
}

fn delete_monkey(m: MonkeyInstance) {
    MouseCursor::move_abs(m.x, m.y);
    thread::sleep(time::Duration::from_millis(50));

    MouseButton::LeftButton.press();
    MouseButton::LeftButton.release();

    if(m.x >)
}

enum Monkeys {
    Ace,
    TackShooter
}