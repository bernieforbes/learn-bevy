use bevy::prelude::*;

pub mod events;
mod game;
mod main_menu;
mod systems;

use game::GamePlugin;
use main_menu::MainMenuPlugin;

use systems::*;

fn main() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins)
        // My Plugins
        .add_plugins((MainMenuPlugin, GamePlugin))
        // State
        .add_state::<AppState>()
        // Startup systems
        .add_systems(Startup, spawn_camera)
        // Systems
        .add_systems(
            Update,
            (
                exit_game,
                handle_game_over,
                transition_to_game_state,
                transition_to_main_menu_state,
            ),
        )
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
