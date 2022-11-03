mod polygon;

use bevy::prelude::*;
use level_generator::{
    random_points,
    voronoi::{alpha_shape, generate_voronoi},
};

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

    let now = std::time::Instant::now();

    let cells = generate_voronoi(random_points(500));

    println!("Generation time: {}s", now.elapsed().as_secs_f32());

    let now = std::time::Instant::now();

    let polygon = alpha_shape(&random_points(500), 1.0);

    let points: Vec<_> = polygon
        .points()
        .iter()
        .map(|p| [p.x as f32, p.y as f32])
        .collect();

    let mesh: Polygon = (&points[..]).into();
    mesh.draw(&mut commands, &mut meshes, &mut materials);

    /*for cell in cells.iter_cells() {
        let points: Vec<_> = cell
            .points()
            .iter()
            .map(|p| [p.x as f32, p.y as f32])
            .collect();

        let mesh: Polygon = (&points[..]).into();
        mesh.draw(&mut commands, &mut meshes, &mut materials);
    }*/

    println!("Display time: {}s", now.elapsed().as_secs_f32());
}
