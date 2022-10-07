mod pong;

use bevy::prelude::*;

use crate::pong::pong::PongPlugin;

fn main() {
    App::new()    
        .add_plugins(DefaultPlugins)
        .add_plugin(PongPlugin)
        .run();
}