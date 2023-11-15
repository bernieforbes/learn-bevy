pub mod enemy;
mod player;
pub mod score;
pub mod sound;
pub mod star;
mod systems;
mod ui;

use crate::events::GameOver;
use crate::AppState;
use bevy::prelude::*;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;
use ui::GameUIPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            // Events
            .add_event::<GameOver>()
            // Plugins
            .add_plugins((
                EnemyPlugin,
                PlayerPlugin,
                ScorePlugin,
                StarPlugin,
                GameUIPlugin,
            ))
            // Systems
            .add_systems(OnEnter(AppState::Game), pause_simulation)
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
            // Exit State Systems
            .add_systems(OnExit(AppState::Game), resume_simulation);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}
