extern crate piston_window;

use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "piston-tutorial",
        [600, 600]
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    while let Some(e) = window.next() {
    
    }
}
