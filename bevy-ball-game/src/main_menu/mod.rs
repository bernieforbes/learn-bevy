mod components;
mod styles;
mod systems;

use bevy::prelude::*;

use crate::AppState;
use systems::layout::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            // OnExit State Systems
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}
