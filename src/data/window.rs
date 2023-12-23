use crate::prelude::*;

pub fn setup_window(mut windows: Query<&mut Window>) {
    for mut window in windows.iter_mut() {
        window.title = "Word-Guessing Game".into();
    }
}
