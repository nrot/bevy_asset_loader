use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader, AssetLoading};
use bevy_progress_tracking::{Progress, Task};

/// This example shows how to track the loading progress of your collections
fn main() {
    let mut app = App::build();
    AssetLoader::new(MyStates::AssetLoading, MyStates::Next)
        .with_collection::<TextureAssets>()
        .with_collection::<AudioAssets>()
        .build(&mut app);
    app.add_state(MyStates::AssetLoading)
        .add_plugins(DefaultPlugins)
        .add_system_set(SystemSet::on_enter(MyStates::Next).with_system(quit.system()))
        .add_system_set(
            SystemSet::on_update(MyStates::AssetLoading)
                .with_system(
                    track_fake_long_task
                        .system()
                        .before(AssetLoading::CheckLoadingState),
                )
                .with_system(
                    print_progress
                        .system()
                        .after(AssetLoading::CheckLoadingState),
                ),
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
    _player: Handle<Texture>,
    #[asset(path = "textures/tree.png")]
    _tree: Handle<Texture>,
    #[asset(path = "textures/female_adventurer.png")]
    _female_adventurer: Handle<Texture>,
}

fn track_fake_long_task(time: Res<Time>, mut progress: ResMut<Progress>) {
    if time.seconds_since_startup() > 5. {
        progress.task(Task::Done);
    } else {
        progress.task(Task::InProgress);
    }
}

fn quit(mut quit: EventWriter<AppExit>) {
    quit.send(AppExit);
}

fn print_progress(mut progress: ResMut<Progress>) {
    progress.finish_frame();
    println!("Current progress: {:?}", progress.progress());
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Copy)]
enum MyStates {
    AssetLoading,
    Next,
}
