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
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
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
    #[cfg(feature = "serialize")]
    if let Err(_) = load_from_path(&mut input, "keybindings.ron") {
        println!("no keybind config found creating default setup"); //just to show the path it took
        create_default_keybindings(&mut input);
        save_to_path(&input, "keybindings.ron").unwrap()
    } else {
        //if it loaded custom keybinds dont add new ones
        println!("keybindings loaded from local file") //just to show the path it took
    }
    #[cfg(not(feature = "serialize"))]
    create_default_keybindings(&mut input);
}

fn create_default_keybindings(input: &mut ResMut<InputMap<ActionEnum>>) {
    //this is so if you want to change default keybindings you dont need to do more then once
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

fn save_to_path(input: &InputMap<ActionEnum>, path: &str) -> std::io::Result<()> {
    let mut data = Vec::new();
    for (action, bindings) in input.get_actions() {
        data.push((action, &bindings.bindings));
    }
    let contents = ron::ser::to_string_pretty(&data, ron::ser::PrettyConfig::default())
        .expect("There was an error making the string");
    std::fs::write(path, contents)?;
    Ok(())
}

fn load_from_path(input: &mut InputMap<ActionEnum>, path: &str) -> std::io::Result<()> {
    let ron_string = std::fs::read_to_string(path)?;
    let actions = ron::from_str(&ron_string).expect("Failed to get actions from ron string");
    input.set_actions(actions);
    //may need to clear self here but i dont really know what that does
    Ok(())
}
