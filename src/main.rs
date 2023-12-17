use ::wordguess::*;

fn kickstart(mut event: EventWriter<state::EndGameEvent>) {
    event.send(state::EndGameEvent);
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, state::StatePlugin, menu::MenuPlugin))
        .add_systems(Startup, (data::setup_words, data::setup_font))
        .add_systems(PostStartup, kickstart)
        .run();
}
