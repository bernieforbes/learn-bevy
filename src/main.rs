use bevy::prelude::*;

fn main() {
    App::new().add_startup_system(setup).run()
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Person {
        name: "Alex".to_string(),
    });
}

#[derive(Component)]
pub struct Person {
    pub name: String,
}
