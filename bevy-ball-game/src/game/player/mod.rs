pub mod components;
mod systems;

use bevy::prelude::*;
use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (player_movement, confine_player_movement).chain())
            .add_systems(Update, (enemy_hit_player, player_hit_star));
    }
}
