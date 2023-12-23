use super::*;
use crate::data::*;
use crate::prelude::*;
use crate::state::*;

pub fn system_game_input(
    mut commands: Commands,
    mut game: ResMut<GameHandler>,
    mut query: Query<(&Tile, &mut Handle<Image>, &mut Text)>,
    mut evr_char: EventReader<ReceivedCharacter>,
    kbd: Res<Input<KeyCode>>,
    mut event: EventWriter<EndGameEvent>,
    img: Res<Images>,
    all_words: Res<AllWords>,
    state: Res<State<GameState>>,
) {
    if *state != GameState::Playing {
        return;
    }

    for e in evr_char.read() {
        let c = e.char;

        if c.is_alphabetic() {
            if game.line < game.y && game.offset < game.x {
                for (tile, _, mut text) in query.iter_mut() {
                    if tile.x == game.offset && tile.y == game.line {
                        debug_println!("Putting char {} at ({}, {})", c, tile.x, tile.y);
                        text.sections[0].value.clear();
                        text.sections[0].value.push(c);
                        text.set_changed();
                    }
                }
                let (y, x) = (game.line, game.offset);
                game.tiles[y][x].character = Some(c);
                game.offset += 1;
            }
        } else if kbd.just_pressed(KeyCode::Back) {
            if game.offset > 0 {
                game.offset -= 1;
                for (tile, _, mut text) in query.iter_mut() {
                    if tile.x == game.offset && tile.y == game.line {
                        text.sections[0].value = "".to_string();
                        text.set_changed();
                    }
                }
                let (y, x) = (game.line, game.offset);
                game.tiles[y][x].character = None;
            }
        } else if kbd.just_pressed(KeyCode::Escape) {
            event.send(EndGameEvent);
        } else if kbd.just_pressed(KeyCode::Return) {
            if game.line < game.y {
                if game.offset >= game.min
                    && game.offset <= game.max
                    && game.test_exists(&all_words)
                {
                    game.colorize();
                    for (tile, mut image, _) in query.iter_mut() {
                        if tile.y == game.line {
                            let game_tile = &mut game.tiles[tile.y][tile.x];
                            match game_tile.color {
                                TileColor::Empty => {
                                    *image = img.tile.clone();
                                }
                                TileColor::Green => {
                                    *image = img.green.clone();
                                }
                                TileColor::Yellow => {
                                    *image = img.yellow.clone();
                                }
                                TileColor::Red => {
                                    *image = img.red.clone();
                                }
                            }
                        }
                    }
                    game.line += 1;
                    game.offset = 0;
                }
            }
        }
    }
}
