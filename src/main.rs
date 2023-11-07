use bevy::prelude::*;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_system(print_names)
        .run()
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Person {
        name: "Alex".to_string(),
    });

    commands.spawn(Person {
        name: "Bob".to_string(),
    });

    commands.spawn(Person {
        name: "Charlie".to_string(),
    });

    commands.spawn(Person {
        name: "David".to_string(),
    });

    commands.spawn(Person {
        name: "Ellen".to_string(),
    });
}

pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name: {}", person.name);
    }
}

#[derive(Component)]
pub struct Person {
    pub name: String,
}
