#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PongState {
    SettingUpUI,
    LoadingAssets,
    InGame,
    Paused,
}
