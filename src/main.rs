use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PeoplePlugin)
        .run()
}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(print_names)
            .add_system(people_with_jobs)
            .add_system(people_ready_for_hire)
            .add_system(person_job_function);
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Person {
            name: "Bob".to_string(),
        },
        Employed { job: Job::Doctor },
    ));
    commands.spawn((
        Person {
            name: "Sam".to_string(),
        },
        Employed {
            job: Job::FireFighter,
        },
    ));
    commands.spawn((
        Person {
            name: "Jason".to_string(),
        },
        Employed { job: Job::Lawyer },
    ));
    commands.spawn(Person {
        name: "James".to_string(),
    });
}

pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name {}", person.name)
    }
}

#[derive(Component)]
pub struct Person {
    pub name: String,
}

#[derive(Component)]
pub struct Employed {
    pub job: Job,
}

#[derive(Debug)]
pub enum Job {
    Doctor,
    FireFighter,
    Lawyer,
}

pub fn people_with_jobs(person_query: Query<&Person, With<Employed>>) {
    for person in person_query.iter() {
        println!("{} has a job", person.name)
    }
}

pub fn people_ready_for_hire(person_query: Query<&Person, Without<Employed>>) {
    for person in person_query.iter() {
        println!("{} is ready for hire", person.name)
    }
}

pub fn person_job_function(person_query: Query<(&Person, &Employed)>) {
    for (person, employed) in person_query.iter() {
        let job_name = match employed.job {
            Job::Doctor => "Doctor",
            Job::FireFighter => "FireFighter",
            Job::Lawyer => "Lawyer",
        };
        println!("{0} is a {1}", person.name, job_name)
    }
}
