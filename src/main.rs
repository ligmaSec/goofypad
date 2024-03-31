use x11::xlib::{XSelectInput, XNextEvent};
use x11_keypress_detect::get_display;
//use std::thread;
fn main() {
    
    let display = get_display();
    
    unsafe{
        let mut event = std::mem::zeroed();
        XNextEvent(display, &mut event);
        match event.get_type() {
            x11::xlib::KeyPress => {
                println!("Key Pressed");
            }
            x11::xlib::KeyRelease => {
                println!("Key Released");
            }
            _ => {
                println!("Other Event");
            }
        }


    }


}
