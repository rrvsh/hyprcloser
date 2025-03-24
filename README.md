# hyprcloser

This crate tracks the current and previous window in Hyprland and closes the previous window when it loses focus.

# Installation

1. Clone the repo.
2. Run `cargo run`.

# Usage

1. Open `clipse`.
2. Perform an action that causes clipse to lose focus. It will now close.

# Considerations

- If running the program to be automatically closed in kitty, the configuration option `confirm_os_window_close` must be set to `0` or else kitty will ask to confirm if you want to close the window, requiring an extra keypress.
