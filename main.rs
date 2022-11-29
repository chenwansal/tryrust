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
fn sys_keyboard_input(input: Res<Input<KeyCode>>) {
    if input.pressed(KeyCode::A) {
        println!("A is pressed");
    }

    if input.just_pressed(KeyCode::B) {
        println!("B is just pressed");
    }

    if input.just_released(KeyCode::C) {
        println!("C is just released");
    }

}

fn sys_add_person(mut commands: Commands) {
    commands.spawn((ComPerson, ComName("Jack".to_string())));
}

fn sys_greetings(time: Res<Time>, mut timer: ResMut<ResTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("Hello, world! {}", time.elapsed_seconds().to_string());
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
        .add_system(sys_keyboard_input)
        .add_plugins(DefaultPlugins)
        .add_plugin(PluginHello)
        .run();
}
