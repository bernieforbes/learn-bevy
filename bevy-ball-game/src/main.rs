use bevy::prelude::*;

pub mod enemy;
pub mod events;
mod player;
pub mod score;
pub mod sound;
pub mod star;
mod systems;

use events::*;
use systems::*;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<GameOver>()
        .add_plugins((EnemyPlugin, PlayerPlugin, ScorePlugin, StarPlugin))
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, (exit_game, handle_game_over))
        .run();
}
