use ang::Angle;
use bevy::prelude::*;
use bevy::reflect::erased_serde::__private::serde::__private::de;
use bevy::window::PresentMode;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_polyline::prelude::{Polyline, PolylineBundle, PolylineMaterial};
use bevy_polyline::PolylinePlugin;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::{
    prelude::{RapierPhysicsPlugin, RigidBody},
    render::RapierDebugRenderPlugin,
};
mod third_person;
use third_person::{ThirdPersonCamera, ThirdPersonController, ThirdPersonControllerPlugin};

use bevy::input::mouse::MouseButton;
use bevy_inspector_egui::Inspectable;

const THROTTLE: f32 = 2.;
const MAX_THRUST: f32 = 1.;

pub struct Movement {
    angle: Angle<f32>,
}

// mod camera;
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            present_mode: PresentMode::AutoNoVsync,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(PolylinePlugin)
        .add_plugin(ThirdPersonControllerPlugin)
        .add_startup_system(setup)
        // .add_system(spaceship_movement)
        .run();
}

#[derive(Component, Inspectable)]
pub struct SpaceShip;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: ResMut<AssetServer>,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
) {
    //polylines
    commands.spawn_bundle(PolylineBundle {
        polyline: polylines.add(Polyline {
            vertices: vec![-Vec3::ONE, Vec3::ONE],
        }),
        material: polyline_materials.add(PolylineMaterial {
            width: 5.0,
            color: Color::RED,
            perspective: true,
            ..Default::default()
        }),
        ..Default::default()
    });

    // Plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(0.0, -0.5, 0.0),
            ..default()
        })
        .insert(Collider::cuboid(50.0, 0.05, 50.0));

    let mass_properties = MassProperties {
        local_center_of_mass: Vec3::new(1.0, 2.0, 3.0),
        mass: 1000.0,
        principal_inertia: Vec3::new(0.5, 0.6, 0.7),
        ..Default::default()
    };

    commands
        .spawn_bundle(SceneBundle {
            scene: asset_server.load("models/deer.glb#Scene0"),
            transform: Transform {
                translation: Vec3::new(0., 10., 0.),
                scale: Vec3::new(1.1, 1.1, 1.1),
                ..default()
            },
            ..default()
        })
        .insert(ThirdPersonController::default())
        .insert(RigidBody::KinematicPositionBased)
        .insert(Velocity::zero())
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Collider::capsule(Vec3::Y * 0.0, Vec3::Y * 0.85, 0.5))
        .insert(Ccd { enabled: true });

    // Light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 15000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Camera
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(ThirdPersonCamera {
            target_offset: Vec3::Y * 1.5,
            position_offset: Vec3::Y * 0.8,
            ..default()
        });

    let movement_params = Movement {
        angle: Angle::Radians(0.),
    };
    commands.insert_resource(movement_params);
}

// fn spaceship_movement(
//     mut ext_forces: Query<(&mut ExternalForce, &mut Velocity)>,
//     key_input: ResMut<Input<KeyCode>>,
// ) {
//     for (mut ext_force, mut velocity) in ext_forces.iter_mut() {
//         if key_input.pressed(KeyCode::Space) {
//             let thrust = THROTTLE * MAX_THRUST * Vec3::new(1., 0., 0.);
//             ext_force.force += thrust;
//         }
//     }
// }

// fn tp_camera_follow_player(
//     mut camera: Query<&mut Transform, (With<ThirdPersonCamera>, Without<SpaceShip>)>,
//     mut player_spaceship: Query<&mut Transform, (With<SpaceShip>, Without<ThirdPersonCamera>)>,
// ) {
//     let mut camera_transform = camera.single_mut();
//     let player_spaceship_transform = player_spaceship.single_mut();

//     let x = player_spaceship_transform.translation.x;
//     let y = player_spaceship_transform.translation.y;
//     let z = 0.;

//     *camera_transform =
//         Transform::from_xyz(x - 100., y + 50., z).looking_at(Vec3::new(x, y, z), Vec3::Y);
// }

// fn camera_movement_system(
//     mut camera: Query<&mut Transform, With<Camera>>,
//     key_input: ResMut<Input<KeyCode>>,
//     mouse_input: ResMut<Input<MouseButton>>,
// ) {
//     if key_input.pressed(KeyCode::A) {
//         for mut cam in camera.iter_mut() {
//             cam.translation.x -= 1.;
//         }
//     }

//     if key_input.pressed(KeyCode::D) {
//         for mut cam in camera.iter_mut() {
//             cam.translation.x += 1.;
//         }
//     }

//     if key_input.pressed(KeyCode::W) {
//         for mut cam in camera.iter_mut() {
//             cam.translation.y += 1.;
//         }
//     }

//     if key_input.pressed(KeyCode::S) {
//         for mut cam in camera.iter_mut() {
//             cam.translation.y -= 1.;
//         }
//     }

//     if key_input.pressed(KeyCode::K) {
//         for mut cam in camera.iter_mut() {
//             cam.translation.z += 1.;
//         }
//     }

//     if key_input.pressed(KeyCode::L) {
//         for mut cam in camera.iter_mut() {
//             cam.translation.z -= 1.;
//         }
//     }

//     for mut cam in camera.iter_mut() {
//         cam.look_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y);
//     }
// }
