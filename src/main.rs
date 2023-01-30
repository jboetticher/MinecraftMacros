use std::{thread, sync::{Arc, Mutex}};
use inputbot::{KeybdKey::F10Key};

mod right_click;
use crate::right_click::RightClick;

#[macro_use]
extern crate lazy_static;

// Creates a list of scripts for the users to use!
// @dev Add scripts to this Vec to add them as options in the CLI
lazy_static! {
    static ref MC_SCRIPTS: Vec<Box<dyn MinecraftMacro + Sync>> = vec![
        Box::new(RightClick::default()),
        Box::new(RightClick::default()),
        Box::new(RightClick::default())
    ];
}

trait MinecraftMacro {
    fn macro_loop(&self);
    fn name(&self) -> String;
    fn desc(&self) -> String;
}

fn main() {
    let macro_enabled: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));

    println!("Use F10 to enable / disable the Macro");

    // Prints out a menu for each of the scripts available
    for i in 0..MC_SCRIPTS.len() {
        println!("{:?}", MC_SCRIPTS[i].name());
    }

    // The macro thread
    let c_enabled = macro_enabled.clone();
    thread::spawn(move || {
        loop {            
            if *c_enabled.lock().unwrap() {
                MC_SCRIPTS[0].macro_loop();        
            }
        }
    });

    // Enabling / Disabling of the macro
    let c_enabled = macro_enabled.clone();
    F10Key.bind(move || {
        let l = c_enabled.lock();
        let mut v = l.unwrap();
        *v = !*v;

        if *v {
            println!("Macro enabled!");
        }
        else {
            println!("Macro disabled!");
        }
    });

    inputbot::handle_input_events();
}
