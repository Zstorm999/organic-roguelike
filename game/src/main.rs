mod polygon;

use bevy::prelude::*;
use level_generator::{random_points, voronoi::generate_voronoi};

use polygon::Polygon;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    let cells = generate_voronoi(random_points(30));

    for cell in cells.iter_cells() {
        let points: Vec<_> = cell.iter_vertices().collect();

        let mesh: Polygon = (&points[..]).into();
        mesh.draw(&mut commands, &mut meshes, &mut materials);
    }

    /*let points: Vec<_> = vec![
        [0.5, 0.0],
        [0.5, 0.1],
        [0.42, 0.71],
        [0.0, 1.0],
        [-0.5, -0.5],
    ];
    let mesh: Polygon = (&points[..]).into();

    mesh.draw(&mut commands, &mut meshes, &mut materials);*/
}
