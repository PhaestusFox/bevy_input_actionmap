use bevy::prelude::*;
use bevy_input_actionmap::*;

fn main() {
    App::build()
        .insert_resource(GameState { current_mode: ActionMode::A })
        .add_plugins(DefaultPlugins)
        .add_plugin(ActionPlugin::<ActionEnum>::default())
        .add_startup_system(setup.system())
        .add_system(run_commands.system())
        .run();
}

#[derive(Hash, PartialEq, Eq, Clone)]
enum ActionEnum {
    A,
    B,
    Switch,
}

#[derive(Hash, PartialEq, Eq, Clone)]
enum ActionMode {
    A,
    B,
}

struct GameState {
    current_mode: ActionMode
}

fn setup(mut input: ResMut<InputMap<ActionEnum>>) {
    input
    .bind(ActionEnum::A, KeyCode::A)
    .bind(ActionEnum::B, KeyCode::B)
    .bind(ActionEnum::Switch, KeyCode::C);
}

fn run_commands(mut game_state: ResMut<GameState>, input: Res<InputMap<ActionEnum>>) {
    if game_state.current_mode == ActionMode::A && input.just_active(ActionEnum::A) {
        println!("A");
    }
    if game_state.current_mode == ActionMode::B && input.just_active(ActionEnum::B) {
        println!("B");
    }
    if input.just_active(ActionEnum::Switch) {
        println!("Switch");
        if game_state.current_mode == ActionMode::A {
            game_state.current_mode = ActionMode::B
        } else {
            game_state.current_mode = ActionMode::A
        }
    }
}
