use hyprland::event_listener::EventListener;

fn main() -> hyprland::Result<()> {
    let mut event_listener = EventListener::new();

    event_listener.add_active_window_changed_handler(|data| {
        if let Some(v) = data {
            println!("{:?}", v.class)
        }
    });
    event_listener.start_listener()?;

    Ok(())
}
