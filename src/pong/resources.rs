use bevy::{prelude::*, gltf::Gltf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PongState {
    LoadingAssets,
    LoadingUI,
    InGame,
    Paused,
}

pub struct ProjectionImages(Handle<Image>, Handle<Image>, Handle<Image>);

impl ProjectionImages {

    pub fn new(xw: Handle<Image>, yw: Handle<Image>, zw: Handle<Image>) -> Self {
        Self(xw, yw, zw)
    }

    pub fn unpack(&self) -> (Handle<Image>, Handle<Image>, Handle<Image>) {
        (self.0.clone(), self.1.clone(), self.2.clone())
    }
}

pub struct GltfModel(pub Handle<Gltf>);
