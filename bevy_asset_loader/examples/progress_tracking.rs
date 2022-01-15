use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader, AssetLoading};
use bevy_loading::{LoadingPlugin, ProgressCounter};

/// This example shows how to track the loading progress of your collections
fn main() {
    let mut app = App::new();
    AssetLoader::new(MyStates::AssetLoading)
        .with_collection::<TextureAssets>()
        .with_collection::<AudioAssets>()
        .build(&mut app);
    app.add_state(MyStates::AssetLoading)
        .add_plugins(DefaultPlugins)
        .add_plugin(LoadingPlugin::new(MyStates::AssetLoading).continue_to(MyStates::Next))
        .add_system_set(SystemSet::on_enter(MyStates::Next).with_system(quit))
        .add_system_set(
            SystemSet::on_update(MyStates::AssetLoading).with_system(track_fake_long_task),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::on_update(MyStates::AssetLoading).with_system(print_progress),
        )
        .run();
}

#[derive(AssetCollection)]
struct AudioAssets {
    #[asset(path = "audio/background.ogg")]
    _background: Handle<AudioSource>,
    #[asset(path = "audio/plop.ogg")]
    _plop: Handle<AudioSource>,
}

#[derive(AssetCollection)]
struct TextureAssets {
    #[asset(path = "textures/player.png")]
    _player: Handle<Image>,
    #[asset(path = "textures/tree.png")]
    _tree: Handle<Image>,
    #[asset(path = "textures/female_adventurer.png")]
    _female_adventurer: Handle<Image>,
}

fn track_fake_long_task(time: Res<Time>, progress: Res<ProgressCounter>) {
    if time.seconds_since_startup() > 5. {
        progress.manually_tick(true);
    } else {
        progress.manually_tick(false);
    }
}

fn quit(mut quit: EventWriter<AppExit>) {
    quit.send(AppExit);
}

fn print_progress(progress: Res<ProgressCounter>) {
    println!("Current progress: {:?}", progress.progress());
}

#[derive(Component, Clone, Eq, PartialEq, Debug, Hash, Copy)]
enum MyStates {
    AssetLoading,
    Next,
}
