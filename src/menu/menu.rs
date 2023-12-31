use crate::data::*;
use crate::prelude::*;
use crate::state::*;

#[derive(Debug, Clone, Component)]
pub struct InputBox;

#[derive(Debug, Clone, Event)]
pub struct SpawnInputBoxEvent;

#[derive(Debug, Clone, Event)]
pub struct DespawnInputBoxEvent;

pub fn system_spawn_input_box(
    mut commands: Commands,
    mut event: EventReader<SpawnInputBoxEvent>,
    cascadia: Res<Cascadia>,
) {
    for _ in event.read() {
        commands.spawn((
            InputBox,
            Text2dBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "".to_string(),
                            style: TextStyle {
                                font: cascadia.font.clone(),
                                font_size: 50.0,
                                color: Color::WHITE,
                            },
                        },
                        TextSection {
                            value: "Input Range {min>=3}-{max<=14}".to_string(),
                            style: TextStyle {
                                font: cascadia.font.clone(),
                                font_size: 50.0,
                                color: Color::CYAN,
                            },
                        },
                    ],
                    ..Default::default()
                },
                ..Default::default()
            },
        ));
    }
}

pub fn system_despawn_input_box(
    mut commands: Commands,
    mut event: EventReader<DespawnInputBoxEvent>,
    query: Query<Entity, With<InputBox>>,
) {
    for _ in event.read() {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn system_input_box(
    mut query: Query<(&mut InputBox, &mut Text)>,
    mut evr_char: EventReader<ReceivedCharacter>,
    kbd: Res<Input<KeyCode>>,
    state: Res<State<GameState>>,
) {
    if *state != GameState::Settings {
        return;
    }

    match query.iter_mut().next() {
        Some((mut _input_box, mut text)) => {
            for ev in evr_char.read() {
                if kbd.just_pressed(KeyCode::Back) {
                    text.sections[0].value.pop();
                } else if ev.char.is_numeric() || ev.char == '-' {
                    text.sections[0].value.push(ev.char);
                    if text.sections.len() > 1 {
                        text.sections.remove(1);
                    }
                }
            }
        }
        None => return,
    }
}

pub fn system_start_game(
    mut query: Query<(&mut InputBox, &mut Text)>,
    mut event: EventWriter<StartGameEvent>,
    kbd: ResMut<Input<KeyCode>>,
    state: Res<State<GameState>>,
) {
    if *state != GameState::Settings {
        return;
    }

    for (_input_box, text) in query.iter_mut() {
        if kbd.just_pressed(KeyCode::Return) {
            let parts: Vec<_> = text.sections[0].value.split('-').collect();
            if parts.len() != 2 {
                return;
            }

            let min;
            let max;

            match parts[0].parse::<usize>() {
                Ok(v) => min = v,
                Err(_) => return,
            }
            if min < 3 {
                return;
            }
            match parts[1].parse::<usize>() {
                Ok(v) => max = v,
                Err(_) => return,
            }
            if max > 14 {
                return;
            }
            if max < min {
                return;
            }

            event.send(StartGameEvent { min, max });
        }
    }
}

pub fn system_start_game_event_listener(
    mut event: EventReader<StartGameEvent>,
    mut despawn_input_box: EventWriter<DespawnInputBoxEvent>,
    mut state: ResMut<NextState<GameState>>,
) {
    for _ in event.read() {
        state.set(GameState::Playing);
        despawn_input_box.send(DespawnInputBoxEvent);
    }
}

pub fn system_end_game_event_listener(
    mut event: EventReader<EndGameEvent>,
    mut spawn_input_box: EventWriter<SpawnInputBoxEvent>,
    mut state: ResMut<NextState<GameState>>,
) {
    for _ in event.read() {
        state.set(GameState::Settings);
        spawn_input_box.send(SpawnInputBoxEvent);
    }
}
