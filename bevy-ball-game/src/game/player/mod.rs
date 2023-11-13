pub mod components;
mod systems;

use bevy::prelude::*;
use systems::*;

use super::SimulationState;
use crate::AppState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                (player_movement, confine_player_movement)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running))
                    .chain(),
            )
            .add_systems(
                Update,
                (enemy_hit_player, player_hit_star)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}
