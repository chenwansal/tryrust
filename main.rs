use bevy::prelude::*;

// ==== Components ====
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

// ==== Systems ====
fn sys_add_person(mut commands: Commands) {
    commands.spawn((Person, Name("Jack".to_string())));
}

fn sys_hello() {
    println!("Hello, world!");
}

fn sys_greetings(query: Query<&Name>) {
    for name in query.iter() {
        println!("To: {}!", name.0);
    }
}

// ==== Plugins ====
pub struct  PluginHello;
impl Plugin for PluginHello {
    fn build(&self, app: &mut App) {
        app.add_startup_system(sys_add_person)
            .add_system(sys_hello)
            .add_system(sys_greetings);
    }
}

// ==== Main ====
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PluginHello)
        .run();
}
