use hyprland::event_listener::EventListener;

fn main() -> hyprland::Result<()> {
    let mut event_listener = EventListener::new();

    event_listener.add_active_window_changed_handler(|data| println!("{data:#?}"));
    event_listener.start_listener()?;

    Ok(())
}
