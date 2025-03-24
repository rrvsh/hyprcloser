use hyprland::event_listener::EventListener;
use std::cell::RefCell;

fn main() -> hyprland::Result<()> {
    let mut event_listener = EventListener::new();
    let prev_window = RefCell::new(String::new());
    let curr_window = RefCell::new(String::new());

    event_listener.add_active_window_changed_handler(move |data| {
        if let Some(window) = data {
            let mut prev = prev_window.borrow_mut();
            let mut curr = curr_window.borrow_mut();
            *prev = curr.clone();
            *curr = window.class;
            println!("---");
            println!("prev window: {:?}", *prev);
            println!("curr window: {:?}", *curr);
        }
    });
    event_listener.start_listener()?;

    Ok(())
}
