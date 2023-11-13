pub mod enemy;
mod player;
pub mod score;
pub mod sound;
pub mod star;

use crate::events::GameOver;
use bevy::prelude::*;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<GameOver>()
            // Plugins
            .add_plugins((EnemyPlugin, PlayerPlugin, ScorePlugin, StarPlugin));
    }
}
