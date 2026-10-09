use std::f32::consts::PI;

use bevy::{color::palettes::tailwind::{BLUE_300, BLUE_400}, mesh::{PrimitiveTopology, VertexAttributeValues}, pbr::wireframe::{WireframeConfig, WireframePlugin}, prelude::*, render::render_resource::VertexAttribute};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use noise::{NoiseFn, Perlin};
use terr_gen::gen_mesh;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));
    app.add_plugins(WireframePlugin::default());

    app.add_plugins(PanOrbitCameraPlugin);

    app.add_systems(Startup, startup);
    app.add_systems(Update, toggle_wireframe);
    app.run();
}

fn startup(
    mut c: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    c.spawn((
        Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
        PanOrbitCamera::default()
    ));

    c.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::linear_rgb(1.0, 0.0, 0.0)))
    ));

    c.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
            shadow_maps_enabled: true,
            ..Default::default()
        },
        Transform {
            translation: Vec3::new(0.0, 100.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.0),
            ..Default::default()
        }
    ));

    let terrain = gen_mesh(1000.0, 200);


    c.spawn((
        Transform::from_xyz(0.0, -1.0, 0.0),
        Mesh3d(meshes.add(terrain)),
        MeshMaterial3d(materials.add(Color::WHITE))
    ));

    c.spawn((
        Mesh3d(meshes.add(
        Plane3d::default().mesh().size(1000.0, 1000.0).subdivisions(1).build()
        )),
        Transform::from_xyz(0.0, -20.0, 0.0),
        MeshMaterial3d(materials.add(Color::from(BLUE_400)))
    ));

}

fn toggle_wireframe(
    mut wireframe_config: ResMut<WireframeConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}