#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PongState {
    LoadingAssets,
    InGame,
    Paused,
}
