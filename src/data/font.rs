use crate::prelude::*;

#[derive(Debug, Resource)]
pub struct Cascadia {
    pub font: Handle<Font>,
}

impl Cascadia {
    fn new(asset_server: Res<AssetServer>) -> Self {
        Self {
            font: asset_server.load("fonts/CascadiaCode.ttf"),
        }
    }
}

pub fn setup_font(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Cascadia::new(asset_server));
}
