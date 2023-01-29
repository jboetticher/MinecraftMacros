use std::{thread, time, sync::{Arc, Mutex}};
use inputbot::{MouseButton, KeybdKey::F10Key};

fn main() {
    let macro_enabled: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));

    // The macro thread
    let c_enabled = macro_enabled.clone();
    thread::spawn(move || {
        loop {            
            if *c_enabled.lock().unwrap() {
                MouseButton::LeftButton.press();
                MouseButton::LeftButton.release();
                thread::sleep(time::Duration::from_millis(100));
                println!("wahoo");
            }
        }
    });

    // Enabling / Disabling of the macro
    let c_enabled = macro_enabled.clone();
    F10Key.bind(move || {
        let l = c_enabled.lock();
        let mut v = l.unwrap();
        *v = !*v;
    });

    inputbot::handle_input_events();
}
