use x11::xlib::{XNextEvent, XOpenDisplay};
//use std::thread;
fn main() {
    unsafe { 
        let display = XOpenDisplay(std::ptr::null());
        if display.is_null() {
            panic!("Cannot open display");
        }

        println!("Display opened {:?}", display);

    loop { 
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


}
