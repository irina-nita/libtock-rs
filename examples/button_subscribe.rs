#![no_std]

use core::fmt::Write;
use futures::future;
use libtock::buttons;
use libtock::buttons::ButtonState;
use libtock::console::Console;

// FIXME: Hangs up when buttons are pressed rapidly - problem in console?
async fn main() {
    let mut console = Console::new();

    let mut with_callback = buttons::with_callback(|button_num: usize, state| {
        writeln!(
            console,
            "Button: {} - State: {}",
            button_num,
            match state {
                ButtonState::Pressed => "pressed",
                ButtonState::Released => "released",
            }
        )
        .unwrap();
    });

    let mut buttons = with_callback.init().unwrap();

    for mut button in &mut buttons {
        button.enable().unwrap();
    }

    future::pending().await
}
