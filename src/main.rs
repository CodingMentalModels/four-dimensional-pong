mod pong;

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use pong::ui::UIPlugin;

use crate::pong::pong::PongPlugin;
use crate::pong::resources::*;

fn main() {
    App::new()    
        .add_plugins(DefaultPlugins)
        .add_loopless_state(PongState::SettingUpUI)
        .add_plugin(UIPlugin)
        .add_plugin(PongPlugin)
        .run();
}