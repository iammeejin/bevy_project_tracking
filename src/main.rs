use bevy::prelude::*;
use bevy_mod_picking::*;


fn main() {
    App::new()
    //Window Setup
      .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
          resolution: (640.0, 400.0).into(),
          title: "Project Tracking".to_string(),
          ..default()
        }),
        ..default()
      }))
      //Mod Picking
      .add_plugins(DefaultPickingPlugins)
      .add_startup_system(setup)
      .run();
  }

/// set up a simple scene with a "parent" cube and a "child" cube
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let cube_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        ..default()
    });

    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(10.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // parent cube
    commands
        .spawn((
            PbrBundle {
                mesh: cube_handle.clone(),
                material: cube_material_handle.clone(),
                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            // child cube
            parent.spawn(PbrBundle {
                mesh: cube_handle,
                material: cube_material_handle,
                transform: Transform::from_xyz(0.0, 0.0, 3.0),
                ..default()
            });
        });
    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 5.0, -4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}