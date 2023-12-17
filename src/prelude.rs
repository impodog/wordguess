pub use std::collections::{HashMap, HashSet};
pub use std::error::Error;
pub use std::fs::File;
pub use std::io::{Read, Write};

pub use bevy::prelude::*;

pub use serde::{Deserialize, Serialize};

#[macro_export]
macro_rules! impl_send_sync {
    ($($type:ty),*) => {
        $(
            unsafe impl Send for $type {}
            unsafe impl Sync for $type {}
        )*
    };
}
