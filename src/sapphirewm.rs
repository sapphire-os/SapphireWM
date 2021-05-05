use x11_dl::xlib;
mod xwrap;

fn main() {
    // Init window system and print all active windows
    let xw = xwrap::XWrap::new();
    xw.init();

    let windows = xwrap::WaWindow::find_all(&xw);
    for window in windows {
        println!("WINDOW: {:#?} ", window);
    }

    // Event handling
    loop {
        let raw_event = xw.get_next_event();
        match raw_event.get_type() {
            xlib::ClientMessage => { 
                let event = xlib::XClientMessageEvent::from(raw_event);
                println!("ClientMessage: {:#?} ", event);
            }
            xlib::ButtonPress => { 
                let event = xlib::XButtonPressedEvent::from(raw_event);
                println!("ButtonPress: {:#?} ", event);
            }
            _ => {}
        }
    }
}
