use crate::prelude::*;

#[derive(Resource)]
pub struct Images {
    pub tile: Handle<Image>,
    pub red: Handle<Image>,
    pub green: Handle<Image>,
    pub yellow: Handle<Image>,
    pub cyan: Handle<Image>,
    pub win: Handle<Image>,
    pub cyan_win: Handle<Image>,
}

pub fn setup_images(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tile = asset_server.load("images/tile.png");
    let red = asset_server.load("images/red.png");
    let green = asset_server.load("images/green.png");
    let yellow = asset_server.load("images/yellow.png");
    let cyan = asset_server.load("images/cyan.png");
    let win = asset_server.load("images/win.png");
    let cyan_win = asset_server.load("images/cyan_win.png");
    commands.insert_resource(Images {
        tile,
        red,
        green,
        yellow,
        cyan,
        win,
        cyan_win,
    });
}
