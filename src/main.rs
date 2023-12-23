use ::wordguess::*;

fn kickstart(mut event: EventWriter<state::EndGameEvent>) {
    event.send(state::EndGameEvent);
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            state::StatePlugin,
            menu::MenuPlugin,
            game::GamePlugin,
        ))
        .add_systems(
            Startup,
            (
                data::setup_words,
                data::setup_font,
                data::setup_images,
                data::setup_window,
            ),
        )
        .add_systems(PostStartup, (kickstart,))
        .run();
}
