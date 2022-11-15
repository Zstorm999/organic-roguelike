mod polygon;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use level_generator::{
    random_points,
    voronoi::{alpha_shape, generate_voronoi},
    Point,
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

    let points = random_points(20);

    let cells = generate_voronoi(points.clone());

    println!("Generation time: {}s", now.elapsed().as_secs_f32());

    let now = std::time::Instant::now();

    for (i, cell) in cells.iter_cells().enumerate() {
        let points: Vec<_> = cell
            .points()
            .iter()
            .map(|p| [p.x as f32, p.y as f32])
            .collect();

        if points.len() == 0 {
            continue;
        }

        //println!("Generated points {}", points.len());

        let mesh: Polygon = (&points[..]).into();

        //println!("Created polygon");

        mesh.draw(
            &mut commands,
            &mut meshes,
            &mut materials,
            Color::Rgba {
                red: 0.0,
                green: (255.0 - 4.0 * i as f32) / 255.0,
                blue: 0.0,
                alpha: 1.0,
            },
        );

        //println!("Drawn")
    }

    let polygon = alpha_shape(&points, f64::INFINITY);

    let barycenter = polygon
        .points()
        .iter()
        .fold(Point { x: 0.0, y: 0.0 }, |a, point| Point {
            x: a.x + point.x,
            y: a.y + point.y,
        });

    let barycenter = Point {
        x: barycenter.x / polygon.points().len() as f64,
        y: barycenter.y / polygon.points().len() as f64,
    };

    let scale_factor = 1.1;

    let polygon = level_generator::Polygon::from_points(
        polygon
            .points()
            .iter()
            .map(|p| {
                let mut center = Point {
                    x: p.x - barycenter.x,
                    y: p.y - barycenter.y,
                };

                // rescale
                center.x *= scale_factor;
                center.y *= scale_factor;

                // put back in "standard" coordinates
                center.x += barycenter.x;
                center.y += barycenter.y;
                center
            })
            .collect(),
    );

    let vertices: Vec<_> = polygon
        .points()
        .iter()
        .map(|p| [p.x as f32, p.y as f32])
        .collect();

    let mesh: Polygon = (&vertices[..]).into();
    mesh.draw(&mut commands, &mut meshes, &mut materials, Color::PURPLE);

    for p in points {
        commands.spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(1.0).into()).into(),
            transform: Transform::default().with_translation(Vec3::new(
                p.x as f32 * 128.0,
                p.y as f32 * 128.0,
                0.1,
            )),
            material: materials.add(ColorMaterial::from(Color::BLUE)),
            ..default()
        });
    }

    println!("Display time: {}s", now.elapsed().as_secs_f32());
}
