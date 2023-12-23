use super::*;
use crate::data::*;
use crate::prelude::*;
use crate::state::*;

pub fn system_game_input(
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
                for (tile, mut image, mut text) in query.iter_mut() {
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
                for (tile, mut image, mut text) in query.iter_mut() {
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
                    if game.collect_word() == game.ans {
                        for (tile, mut image, _) in query.iter_mut() {
                            if tile.y == game.line {
                                let game_tile = &mut game.tiles[tile.y][tile.x];
                                match game_tile.color {
                                    TileColor::Cyan => {
                                        *image = img.cyan_win.clone();
                                    }
                                    _ => {
                                        *image = img.win.clone();
                                    }
                                }
                            }
                        }
                    } else {
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
                                    TileColor::Cyan => {
                                        *image = img.cyan.clone();
                                    }
                                }
                            }
                        }
                    }
                    game.line += 1;
                    game.offset = 0;
                }
            }
        } else if kbd.just_pressed(KeyCode::Slash) {
            let line = game.y - 1;
            for (tile, mut image, mut text) in query.iter_mut() {
                if tile.y == line && tile.x < game.ans.len() {
                    text.sections[0].value.clear();
                    text.sections[0]
                        .value
                        .push(game.ans.chars().nth(tile.x).unwrap());
                    *image = img.cyan.clone();
                    game.line = game.y;
                }
            }
        } else if kbd.just_pressed(KeyCode::Tab) {
            if game.line < game.y && game.offset < game.x {
                let mut suggest: char = ' ';
                for (tile, mut image, mut text) in query.iter_mut() {
                    if tile.x == game.offset && tile.y == game.line {
                        text.sections[0].value.clear();
                        debug_println!(
                            "Querying suggestion (ans: {}, offset: {})",
                            game.ans,
                            game.offset
                        );
                        if let Some(c_) = game.ans.chars().nth(game.offset) {
                            suggest = c_;
                            debug_println!("Suggestion: {}", suggest);
                            text.sections[0].value.push(suggest);
                            text.set_changed();
                            *image = img.cyan.clone();
                        }
                    }
                }
                let (y, x) = (game.line, game.offset);
                if suggest != ' ' {
                    game.tiles[y][x].character = Some(suggest);
                    game.tiles[y][x].color = TileColor::Cyan;
                    game.offset += 1;
                } else {
                    game.tiles[y][x].character = None;
                }
            }
        }
    }
}
