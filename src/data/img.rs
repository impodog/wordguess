use crate::prelude::*;

#[derive(Resource)]
pub struct Images {
    pub tile: Handle<Image>,
    pub red: Handle<Image>,
    pub green: Handle<Image>,
    pub yellow: Handle<Image>,
}

pub fn setup_images(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tile = asset_server.load("images/tile.png");
    let red = asset_server.load("images/red.png");
    let green = asset_server.load("images/green.png");
    let yellow = asset_server.load("images/yellow.png");
    commands.insert_resource(Images {
        tile,
        red,
        green,
        yellow,
    });
}
