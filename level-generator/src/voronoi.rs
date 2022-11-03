use core::slice::Iter;
use std::collections::HashSet;

use voronator::{
    delaunator::{triangulate, Point},
    polygon::Polygon,
    VoronoiDiagram,
};

pub struct Cells(VoronoiDiagram<Point>);

impl Cells {
    #[inline(always)]
    pub fn iter_cells(&self) -> Iter<Polygon<Point>> {
        self.0.cells().into_iter()
    }
}

pub fn generate_voronoi(points: Vec<Point>) -> Cells {
    let size = 1.0;
    let _clip_polygon = Polygon::from_points(vec![
        Point { x: -size, y: -size },
        Point { x: size, y: -size },
        Point { x: size, y: size },
        Point { x: -size, y: size },
    ]);

    //let alpha_bound = alpha_shape(&points, 0.6);

    Cells(VoronoiDiagram::with_bounding_polygon(points, &_clip_polygon).unwrap())
}

pub fn alpha_shape(points: &[Point], alpha: f64) -> Polygon<Point> {
    assert!(points.len() > 3);

    let t = triangulate(points).expect("Unable to perform Delaunay triangulation");

    let mut edges = HashSet::new();

    fn add_edge(edges: &mut HashSet<(usize, usize)>, i: usize, j: usize) {
        if edges.contains(&(i, j)) || edges.contains(&(i, j)) {
            assert!(edges.contains(&(j, i)));
            edges.remove(&(j, i));
            return;
        }

        edges.insert((i, j));
    }

    println!("{} points", points.len());
    println!("{:?} triangles", t.len());

    // waiting for iter_array_chunk to make it to standard (only nightly for now and I don't wanna go there)
    for i in 0..t.len() {
        let ia = t.triangles[3 * i];
        let ib = t.triangles[3 * i + 1];
        let ic = t.triangles[3 * i + 2];

        let pa = points[ia].clone();
        let pb = points[ib].clone();
        let pc = points[ic].clone();

        let a_0 = (pa.x - pb.x).powi(2) + (pa.y - pb.y).powi(2);
        let b_0 = (pb.x - pc.x).powi(2) + (pb.y - pc.y).powi(2);
        let c_0 = (pc.x - pa.x).powi(2) + (pc.y - pa.y).powi(2);

        let a = f64::sqrt(a_0);
        let b = f64::sqrt(b_0);
        let c = f64::sqrt(c_0);

        let s = (a + b + c) / 2.0;
        let area = f64::sqrt(s * (s - a) * (s - b) * (s - c));
        let circum_r = a * b * c / (4.0 * area);

        // pretty sure we can skip the square roots here
        // to test and check in wolfram alpha
        /*let area = -a_0.powi(2) - b_0.powi(2) - c_0.powi(2)
            + (2.0 * a_0 * b_0)
            + (2.0 * b_0 * c_0)
            + (2.0 * c_0 + a_0);

        let circum_r_sqr = (a_0 * b_0 * c_0) / area;*/

        if circum_r < alpha {
            add_edge(&mut edges, ia, ib);
            add_edge(&mut edges, ib, ic);
            add_edge(&mut edges, ic, ia);
        }
    }

    // now we have all the edges in the set
    // we need to create a polygon

    // really disgusting O(n²) loop, need rework
    // this should actually be fine since we don’t have a lot of points

    println!("{} edges", edges.len());

    let mut vertices: Vec<Point> = Vec::new();
    let mut current_edge = edges.iter().nth(0).unwrap();

    vertices.push(points[current_edge.0].clone());
    vertices.push(points[current_edge.1].clone());

    let mut counter = 0;

    loop {
        print!("\riteration {}", counter);
        counter += 1;

        let last_edge = current_edge;

        for edge in &edges {
            if edge.0 == current_edge.1 && edge.1 != current_edge.0 {
                // this is a following edge, but not the same in reverse, which should not happen anyway

                if points[edge.1] == vertices[0] {
                    // we have looped !
                    println!("Polygon with {} vertices", vertices.len());

                    return Polygon::from_points(vertices);
                } else {
                    current_edge = edge;
                    vertices.push(points[current_edge.1].clone())
                }
            }
        }

        if last_edge == current_edge {
            // we went through all edges but none is the next
            println!(
                "\nWARNING: unable to close alpha shape normally, generating a non-existing edge"
            );

            println!("Polygon with {} vertices", vertices.len());

            return Polygon::from_points(vertices);
        }
    }
}
