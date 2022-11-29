use bevy::prelude::*;

use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::sprite::MaterialMesh2dBundle;

/**
    Convex polygon

   The polygon is expected to be centred around (0,0) and convex.
   No verification is performed on the convexity, this is left up to the user
*/
pub struct Polygon {
    _polygon: Mesh,
    _contours: Mesh,
}

impl Polygon {
    pub fn _draw(
        &self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        color: Color,
    ) {
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(self._polygon.clone()).into(),
            transform: Transform::default().with_scale(Vec3::splat(128.0)),
            material: materials.add(ColorMaterial::from(color)),
            ..default()
        });

        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(self._contours.clone()).into(),
            transform: Transform::default().with_scale(Vec3::splat(128.0)),
            material: materials.add(ColorMaterial::from(Color::BLACK)),
            ..default()
        });
    }
}

impl From<&[[f32; 2]]> for Polygon {
    fn from(points: &[[f32; 2]]) -> Self {
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

        let mut polygon = Mesh::new(PrimitiveTopology::TriangleList);
        polygon.set_indices(Some(Indices::U32(indices)));
        polygon.insert_attribute(Mesh::ATTRIBUTE_POSITION, points_3d.clone());
        polygon.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals.clone());
        polygon.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs.clone());

        points_3d.push(points_3d[0]);
        normals.push(normals[0]);
        uvs.push(uvs[0]);

        let mut contours = Mesh::new(PrimitiveTopology::LineStrip);
        contours.insert_attribute(Mesh::ATTRIBUTE_POSITION, points_3d);
        contours.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        contours.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

        Polygon {
            _polygon: polygon,
            _contours: contours,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[should_panic]
    fn panics_if_less_than_three_points() {
        let points = vec![];

        let _: Polygon = (&points[..]).into();
    }

    fn test_shape(points: &[[f32; 2]]) {
        let shape: Polygon = points.into();

        shape
            ._polygon
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
            ._polygon
            .attribute(Mesh::ATTRIBUTE_NORMAL)
            .unwrap()
            .as_float3()
            .unwrap()
            .iter()
            .for_each(|n| assert_eq!(n, &[0.0, 0.0, 1.0]));

        shape
            ._polygon
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
