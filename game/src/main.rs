use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::sprite::MaterialMesh2dBundle;

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

    let mesh = mesh_from_polygon(&vec![
        [0.5, 0.0],
        [0.5, 0.1],
        [0.42, 0.71],
        [0.0, 1.0],
        [-0.5, -0.5],
    ]);

    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(mesh).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        ..default()
    });
}

/**
   Creates a mesh from the vertices of a polygon.

   This function expects the polygon to be convex, centred around (0, 0) and the points to be presented in trigonometric order.
   The position of the first point is however irrelevant (it doesn’t have to lie on any of the two axes).

   # Panics

   Panics if less than 3 points are provided.
*/
fn mesh_from_polygon(points: &[[f32; 2]]) -> Mesh {
    assert!(points.len() > 2);

    let mut points_3d: Vec<_> = Vec::with_capacity(points.len());
    let mut normals: Vec<_> = Vec::with_capacity(points.len());
    let mut uvs: Vec<_> = Vec::with_capacity(points.len());

    for p in points {
        points_3d.push([p[0], p[1], 0.0]);
        normals.push([0.0, 0.0, 1.0]);
        uvs.push([0.0, 0.0]);
    }

    let nb_triangles = points.len() - 2;

    let indices: Vec<_> = (1..)
        .map(|i| [0, i, i + 1])
        .take(nb_triangles)
        .flatten()
        .collect();

    println!("{:?}", indices);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, points_3d);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[should_panic]
    fn panics_if_less_than_three_points() {
        let _ = mesh_from_polygon(&[]);
    }

    fn test_shape(points: &[[f32; 2]]) {
        let shape = mesh_from_polygon(&points);

        shape
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .unwrap()
            .as_float3()
            .unwrap()
            .iter()
            .zip(points.iter())
            .for_each(|(new, original)| {
                assert_eq!(&new[0..2], original);
                assert_eq!(new[2], 0.0);
            });

        shape
            .attribute(Mesh::ATTRIBUTE_NORMAL)
            .unwrap()
            .as_float3()
            .unwrap()
            .iter()
            .for_each(|n| assert_eq!(n, &[0.0, 0.0, 1.0]));

        shape
            .indices()
            .unwrap()
            .iter()
            .zip(
                (1..)
                    .map(|i| [0, i, i + 1])
                    .take(points.len() - 2)
                    .flatten(),
            )
            .for_each(|(new, original)| assert_eq!(new, original))

        // don’t test uvs since we’re only using single colours for now
    }

    #[test]
    fn correctly_builds_triangle() {
        let points = [[0.0, 1.0], [-1.0, 0.0], [1.0, 0.0]];
        test_shape(&points);
    }

    #[test]
    fn correctly_builds_square() {
        let points = [[-0.5, -0.5], [0.5, -0.5], [0.5, 0.5], [-0.5, 0.5]];
        test_shape(&points);
    }
}
