use std::f32::consts::TAU;

use bevy::asset::LoadState;
use bevy::gltf::Gltf;
use bevy::gltf::GltfMesh;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::Extent3d;
use bevy::render::render_resource::TextureDescriptor;
use bevy::render::render_resource::TextureDimension;
use bevy::render::render_resource::TextureFormat;
use bevy::render::render_resource::TextureUsages;
use iyes_loopless::prelude::*;

use crate::pong::components::*;
use crate::pong::resources::*;
use crate::pong::constants::*;

use super::player;
use super::player::Player;

const GLTF_PATH: &str = "four-dimensional-pong.glb";

pub struct LoadAssetsPlugin;

impl Plugin for LoadAssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(
                AmbientLight {
                    color: Color::WHITE,
                    brightness: 1.0 / 2.0,
                }
            )
            .add_enter_system(PongState::LoadingAssets, load_gltf)
            .add_enter_system(PongState::LoadingAssets, instantiate_projection_images)
            .add_system(stage_load_system.run_in_state(PongState::LoadingAssets));
    }
}

// Resources

// End Resources

// Systems


fn load_gltf(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let gltf = asset_server.load(GLTF_PATH);
    if asset_server.get_load_state(gltf.clone()) == LoadState::Failed {
        println!("Immediately failed to load gltf.");
    }
    commands.insert_resource(GltfModel(gltf));
}


fn instantiate_projection_images(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    
    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };    


    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
        },
        ..default()
    };

    image.resize(size); // Fill image.data with zeroes

    let xw_image_handle = images.add(image.clone());
    let yw_image_handle = images.add(image.clone());
    let zw_image_handle = images.add(image.clone());

    commands.insert_resource(ProjectionImages::new(xw_image_handle, yw_image_handle, zw_image_handle));
    
}

fn stage_load_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    model: Res<GltfModel>,
    assets_gltf: Res<Assets<Gltf>>,
    assets_gltf_meshes: Res<Assets<GltfMesh>>,
    projection_images: Res<ProjectionImages>,
) {
    if asset_server.get_load_state(&model.0) == LoadState::Failed {
        println!("Failed to load gltf.");
    }

    if let Some(model_root) = assets_gltf.get(&model.0) {
        let arena = model_root.named_meshes["Arena"].clone();
        let rectangular_arena = model_root.named_meshes["Rectangular Arena"].clone();
        let ball = model_root.named_meshes["Ball"].clone();
        let player_paddle = model_root.named_meshes["Blue Paddle"].clone();
        let opponent_paddle = model_root.named_meshes["Red Paddle"].clone();

        let arena_material = model_root.named_materials["Arena Material"].clone();
        let ball_material = model_root.named_materials["Ball Material"].clone();
        let player_paddle_material = model_root.named_materials["Blue Paddle Material"].clone();
        let opponent_paddle_material = model_root.named_materials["Red Paddle Material"].clone();
        
        commands.spawn_bundle(
            PbrBundle {
                mesh: get_mesh_from_gltf_or_panic(&assets_gltf_meshes, &arena),
                material: arena_material.clone(),
                ..Default::default()
            }
        );

        let arena_rect_transform = Transform::from_xyz(0.0, Y_OFFSET_FOR_PROJECTIONS, 0.0);
        commands.spawn_bundle(
            PbrBundle {
                mesh: get_mesh_from_gltf_or_panic(&assets_gltf_meshes, &rectangular_arena),
                material: arena_material.clone(),
                transform: arena_rect_transform,
                ..Default::default()
            }
        );

        spawn_object_and_projections(
            &mut commands,
            &assets_gltf_meshes,
            &ball,
            &ball_material,
            Vec4::ZERO,
            BallComponent,
            None,
        );
        
        let player_starting_position = Vec4::new(0., 0., -PADDLE_STARTING_OFFSET, -(ARENA_LENGTH / 2.));
        let opponent_starting_position = Vec4::new(0., 0., PADDLE_STARTING_OFFSET, (ARENA_LENGTH / 2.0));

        spawn_object_and_projections(
            &mut commands,
            &assets_gltf_meshes,
            &player_paddle,
            &player_paddle_material,
            player_starting_position,
            PaddleComponent(Player::Blue),
            Some(PlayerInputComponent),
        );


        spawn_object_and_projections(
            &mut commands,
            &assets_gltf_meshes,
            &opponent_paddle,
            &opponent_paddle_material,
            opponent_starting_position,
            PaddleComponent(Player::Red),
            None,
        );

        let x_from_blender = 0.0;
        let y_from_blender = -8.21107;
        let z_from_blender = 4.66824;
        let scalar = 0.5;
        let transform = Transform::from_xyz(x_from_blender*scalar, y_from_blender*scalar, z_from_blender*scalar)
            .looking_at(Vec3::new(0.0, 0., 0.0), Vec3::Y);
        commands.spawn_bundle(
            Camera3dBundle {
                transform: transform,
                ..default()
            }
        );

        let (xw_image_handle, yw_image_handle, zw_image_handle) = projection_images.unpack();

        commands = spawn_cameras_on_images(commands, xw_image_handle, -1, transform, Vec3::new(-DELTA_X_FOR_PROJECTIONS , Y_OFFSET_FOR_PROJECTIONS, 0.));
        commands = spawn_cameras_on_images(commands, yw_image_handle, -2, transform, Vec3::new(0. , Y_OFFSET_FOR_PROJECTIONS, 0.));
        commands = spawn_cameras_on_images(commands, zw_image_handle, -3, transform, Vec3::new(DELTA_X_FOR_PROJECTIONS , Y_OFFSET_FOR_PROJECTIONS, 0.));

        commands.insert_resource(NextState(PongState::LoadingUI));
    }
}


// End Systems

// Helper Functions

fn spawn_object_and_projections(
    commands: &mut Commands,
    assets_gltf_meshes: &Res<Assets<GltfMesh>>,
    mesh: &Handle<GltfMesh>,
    material: &Handle<StandardMaterial>,
    position: Vec4,
    label_component: impl Component + Copy,
    input_component: Option<PlayerInputComponent>,
) {
    // Spawn actual object for the main camera
    spawn_object(
        commands,
        assets_gltf_meshes,
        mesh,
        material,
        position,
        Transform::identity(),
        label_component,
        input_component,
        true
    );

    spawn_object(
        commands,
        assets_gltf_meshes,
        mesh,
        material,
        position,
        Transform::from_translation(Vec3::new(-DELTA_X_FOR_PROJECTIONS, Y_OFFSET_FOR_PROJECTIONS, 0.)),
        label_component,
        input_component,
        false,
    );

    spawn_object(
        commands,
        assets_gltf_meshes,
        mesh,
        material,
        position,
        Transform::from_translation(Vec3::new(0., Y_OFFSET_FOR_PROJECTIONS, 0.)),
        label_component,
        input_component,
        false,
    );

    spawn_object(
        commands,
        assets_gltf_meshes,
        mesh,
        material,
        position,
        Transform::from_translation(Vec3::new(DELTA_X_FOR_PROJECTIONS, Y_OFFSET_FOR_PROJECTIONS, 0.)),
        label_component,
        input_component,
        false,
    );
}

fn spawn_object(
    commands: &mut Commands,
    assets_gltf_meshes: &Res<Assets<GltfMesh>>,
    mesh: &Handle<GltfMesh>,
    material: &Handle<StandardMaterial>,
    position: Vec4,
    projection_transform: Transform,
    label_component: impl Component + Copy,
    input_component: Option<PlayerInputComponent>,
    is_real_object: bool,
) {
    let transform = projection_transform * Transform::from_translation(position.truncate());
    let mut entity_commands = commands.spawn_bundle(
        PbrBundle {
            transform: transform,
            mesh: get_mesh_from_gltf_or_panic(&assets_gltf_meshes, &mesh),
            material: material.clone(),
            ..Default::default()
        }
    );
    entity_commands.insert(label_component)
        .insert(RenderTransformComponent(projection_transform))
        .insert(PositionComponent(position))
        .insert(VelocityComponent(Vec4::ZERO))
        .insert(MaterialHandleComponent(material.clone()))
        .insert(NeedsRenderingComponent);

    match input_component {
        Some(input_component) => {
            entity_commands.insert(input_component);
        },
        None => {}
    };
}

fn get_mesh_from_gltf_or_panic(gltf_mesh_assets: &Res<Assets<GltfMesh>>, gltf_mesh_handle: &Handle<GltfMesh>) -> Handle<Mesh> {
    let gltf_mesh = gltf_mesh_assets.get(&gltf_mesh_handle).expect("The GLTFMesh should exist.");
    gltf_mesh.primitives[0].mesh.clone()
}

fn spawn_cameras_on_images<'a, 'b>(
    mut commands: Commands<'a, 'b>,
    image_handle: Handle<Image>,
    priority: isize,
    transform: Transform,
    translation: Vec3,
) -> Commands<'a, 'b> {
    
    let mut final_transform  = transform;
    final_transform = Transform::from_translation(translation) * final_transform;
    commands.spawn_bundle(
        Camera3dBundle {
            transform: final_transform,
            camera: Camera {
                target: RenderTarget::Image(image_handle.clone()),
                priority: priority,
                ..default()
            },
            ..default()
        }
    );
    
    return commands;
}

// End Helper Functions


#[cfg(test)]
mod test_assets {
    use bevy::{gltf::GltfPlugin, asset::AssetPlugin, window::WindowPlugin, input::InputPlugin};
    use bevy_egui::EguiPlugin;

    use super::*;

    #[test]
    fn test_assets_load() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugin(WindowPlugin)
            .add_plugin(InputPlugin)
            .add_plugin(AssetPlugin)
            .add_plugin(GltfPlugin)
            .add_plugin(EguiPlugin)
            .add_loopless_state(PongState::LoadingAssets)
            .add_asset::<bevy::pbr::prelude::StandardMaterial>()
            .add_asset::<bevy::render::prelude::Mesh>()
            .add_asset::<bevy::scene::Scene>()
            .add_asset::<Image>()
            .add_plugin(LoadAssetsPlugin);

        app.update();
        assert!(app.world.contains_resource::<GltfModel>());
        std::thread::sleep(std::time::Duration::from_millis(100));
        app.update();

        assert!(app.world.contains_resource::<AssetServer>());
        let asset_server = app.world.get_resource::<AssetServer>().expect("AssetServer should exist.");

        let model = app.world.get_resource::<GltfModel>();
        assert!(model.is_some());
        let model = model.unwrap();
        assert!(asset_server.get_load_state(model.0.clone()) != LoadState::Failed);
        assert!(asset_server.get_load_state(model.0.clone()) == LoadState::Loaded);
        assert_eq!(app.world.entities().len(), 6);
        
    }

}