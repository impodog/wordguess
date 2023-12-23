use super::*;
use crate::data::*;
use crate::prelude::*;
use crate::state::*;

pub fn system_spawn_game(
    mut commands: Commands,
    mut event: EventReader<StartGameEvent>,
    window: Query<&Window>,
    img: Res<Images>,
    cascadia: Res<Cascadia>,
    words: Res<CommonWords>,
) {
    for e in event.read() {
        let game = GameHandler::new(e.min, e.max, window.iter().next().unwrap(), &words);
        for y in 0..game.y {
            for x in 0..game.x {
                let tile = TileBundle::new(game.flip, y, x, game.y, game.x, game.size, &cascadia);
                let sprite = SpriteBundle {
                    texture: img.tile.clone(),
                    transform: tile.text.transform,
                    ..Default::default()
                };

                commands.spawn(tile).insert(sprite);
            }
        }
        commands.insert_resource(game);
    }
}

pub fn system_despawn_game(
    mut commands: Commands,
    mut event: EventReader<EndGameEvent>,
    query: Query<Entity, With<Tile>>,
) {
    for _ in event.read() {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
