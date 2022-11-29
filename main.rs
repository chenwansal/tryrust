use bevy::prelude::*;

// ==== Resources ====
#[derive(Resource)]
struct ResTimer(Timer);

// ==== Components ====
#[derive(Component)]
struct ComPerson;

#[derive(Component)]
struct ComName(String);

// ==== Systems ====
fn sys_add_person(mut commands: Commands) {
    commands.spawn((ComPerson, ComName("Jack".to_string())));
}

fn sys_greetings(time: Res<Time>, mut timer: ResMut<ResTimer>, query: Query<&ComName>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("To: {}!", name.0);
        }
    }
}

// ==== Plugins ====
pub struct PluginHello;
impl Plugin for PluginHello {
    fn build(&self, app: &mut App) {
        app.add_startup_system(sys_add_person)
            .add_system(sys_greetings);
    }
}

// ==== Main ====
fn main() {
    let fps = 1.0 / 60.0;
    App::new()
        .insert_resource(ResTimer(Timer::from_seconds(fps, TimerMode::Repeating)))
        .add_plugins(DefaultPlugins)
        .add_plugin(PluginHello)
        .run();
}
