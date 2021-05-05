use x11_dl::xlib;
mod xwrap;

fn main() {

    let xw = xwrap::XWrap::new();

    xw.init();
    let windows = xwrap::WaWindow::find_all(&xw);

    for window in windows {
        println!("WINDOW: {:#?} ", window);
    }

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
            _ => {
                //println!("UNKNOWN EVENT: ");
            }
        }
    }

    //let windows = xw.get_all_windows().unwrap();
    ////pub fn get_window_attrs(&self, window: xlib::Window) -> xlib::XWindowAttributes {
    ////let attrs: Vec<xlib::XWindowAttributes> = windows.into_iter().map(|w| xw.get_window_attrs(w) ).collect();
    //let names: Vec<String> = windows.into_iter().map(|w| xw.get_window_name(w) ).collect();

    //for name in names {
    //    println!("WINDOW: {} ", name);
    //}
    //println!("DONE!");
}
