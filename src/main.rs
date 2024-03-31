use inputbot::{KeySequence, KeybdKey::*, MouseButton::*};
use std::{thread::sleep, time::Duration};

fn main() {
    // Bind the number 1 key your keyboard to a function that types 
    // "Hello, world!" when pressed.
    //TabKey.bind(|| KeySequence("Hello, world!").send());
    CKey.bind(|| println!("Hello, world!"));

    // Bind your caps lock key to a function that starts an autoclicker.
    CapsLockKey.bind(move || {
        while CapsLockKey.is_toggled() {
            LeftButton.press();
            LeftButton.release();

            sleep(Duration::from_millis(30));
        }
    });
    println!("{:?}",inputbot::get_keybd_key('c'));
    // Call this to start listening for bound inputs.
    inputbot::handle_input_events();


}
