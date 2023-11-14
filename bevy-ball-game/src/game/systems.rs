use bevy::prelude::*;

use crate::game::SimulationState;

pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_sim_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match simulation_state.get() {
            SimulationState::Running => {
                next_sim_state.set(SimulationState::Paused);
                println!("Simulation Paused");
            }
            SimulationState::Paused => {
                next_sim_state.set(SimulationState::Running);
                println!("Simulation Running");
            }
        }
    }
}
