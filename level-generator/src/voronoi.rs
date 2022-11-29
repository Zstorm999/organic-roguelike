use core::slice::Iter;
use std::collections::HashSet;

use voronator::{
    delaunator::{triangulate, Coord, Point},
    polygon::Polygon,
    VoronoiDiagram,
};

pub struct Cells(Vec<Polygon<Point>>);

impl Cells {
    pub fn iter_cells(&self) -> Iter<Polygon<Point>> {
        self.0.iter()
    }
}

pub fn generate_voronoi(points: Vec<Point>) -> Cells {
    let size = 2.0;
    let _clip_polygon = Polygon::from_points(vec![
        Point { x: -size, y: -size },
        Point { x: size, y: -size },
        Point { x: size, y: size },
        Point { x: -size, y: size },
    ]);

    let alpha_bound = alpha_shape(&points, f64::INFINITY);

    let barycenter = alpha_bound
        .points()
        .iter()
        .fold(Point { x: 0.0, y: 0.0 }, |a, point| Point {
            x: a.x + point.x,
            y: a.y + point.y,
        });

    let barycenter = Point {
        x: barycenter.x / alpha_bound.points().len() as f64,
        y: barycenter.y / alpha_bound.points().len() as f64,
    };

    let scale_factor = 1.1;

    let extended_bound = Polygon::from_points(
        alpha_bound
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

    print!("[");
    for p in extended_bound.points() {
        print!("{:?}, ", p);
    }

    println!("]");

    Cells(
        VoronoiDiagram::with_bounding_polygon(points, &_clip_polygon)
            .unwrap()
            .cells()
            .into_iter()
            .filter_map(|poly| match is_inside(poly, &extended_bound) {
                Ok(_) => Some(Polygon::from_points(poly.points().to_owned())),
                Err(inside) => None, /*Some(regularize(poly, &extended_bound, &inside))*/
            })
            .to_owned()
            .collect(),
    )
}

pub fn alpha_shape(points: &[Point], alpha: f64) -> Polygon<Point> {
    assert!(points.len() > 3);

    let t = triangulate(points).expect("Unable to perform Delaunay triangulation");

    let mut edges = HashSet::new();

    fn add_edge(edges: &mut HashSet<(usize, usize)>, i: usize, j: usize) {
        if edges.contains(&(i, j)) || edges.contains(&(j, i)) {
            assert!(edges.contains(&(j, i)));
            edges.remove(&(j, i));
            return;
        }

        edges.insert((i, j));
    }

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

    let mut vertices: Vec<Point> = Vec::new();
    let mut current_edge = edges.iter().nth(0).unwrap();

    vertices.push(points[current_edge.0].clone());
    vertices.push(points[current_edge.1].clone());

    loop {
        let last_edge = current_edge;

        for edge in &edges {
            if edge.0 == current_edge.1 && edge.1 != current_edge.0 {
                // this is a following edge, but not the same in reverse, which should not happen anyway

                if points[edge.1] == vertices[0] {
                    // we have looped !
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

            return Polygon::from_points(vertices);
        }
    }
}

fn regularize(
    poly: &Polygon<Point>,
    bounding: &Polygon<Point>,
    inside: &Vec<bool>,
) -> Polygon<Point> {
    assert_eq!(poly.points().len(), inside.len());

    let nb_points = poly.points().len();

    let mut start = 0;

    while inside[start] != true {
        // find the first point inside the bounds
        start += 1;
    }

    let mut points = (0..nb_points)
        .into_iter()
        .cycle()
        .skip(start)
        .take(nb_points);

    // we expect the number of regularized points to be about the same as the old one
    // this is not precise, but an educated guess
    let mut res_poly: Vec<Point> = Vec::with_capacity(nb_points);

    let mut current = points
        .next()
        .expect("HTF did we get a polygon with no points ?");

    loop {
        // we know the current point is inside the polygon, so we add it
        res_poly.push(poly.points()[current].clone());

        let next_inside = loop {
            let next = match points.next() {
                Some(i) => i,
                None => break start, // we have gone full loop, check is already done
            };

            if inside[next] == true {
                break next;
            }
        };

        let after_current = if current == nb_points - 1 {
            0
        } else {
            current + 1
        };

        if next_inside != after_current {
            // there is at least some point outside the bounds inbetween

            let last_outside = if next_inside == 0 {
                nb_points - 1
            } else {
                next_inside - 1
            };

            let out_inter = find_intersection(
                &poly.points()[current],
                &poly.points()[after_current],
                bounding,
            );

            let in_inter = find_intersection(
                &poly.points()[last_outside],
                &poly.points()[next_inside],
                bounding,
            );

            res_poly.push(out_inter.1);
            res_poly.push(in_inter.1);
        }

        if next_inside == current {
            break;
        } else {
            current = next_inside;
        }
    }

    Polygon::from_points(res_poly)
}

fn find_intersection(start: &Point, end: &Point, bounds: &Polygon<Point>) -> (usize, Point) {
    // we need to find the intersection of two straight lines
    // ax + b will be our reference segment
    // cx + d will be the current edge

    let ref_line = Segment::new(start, end);

    for i in 0..bounds.points().len() {
        let j = if i == 0 {
            bounds.points().len() - 1
        } else {
            i - 1
        };

        let e_start = &bounds.points()[j];
        let e_end = &bounds.points()[i];

        let edge_line = Segment::new(e_start, e_end);

        match ref_line.intersects(&edge_line) {
            Some(intersect) => {
                if ref_line.contains(&intersect) && edge_line.contains(&intersect) {
                    return (j, intersect);
                }
            }
            None => {}
        }
    }

    panic!(
        "No intersection found between segment ({:?}, {:?}) and polygon {:?}",
        start,
        end,
        bounds.points()
    )
}

struct Segment {
    r: f64,
    theta: f64,
    r_max: f64,
}

impl Segment {
    fn new(pa: &impl Coord, pb: &impl Coord) -> Self {
        let theta = f64::atan2(pa.y() - pb.y(), pa.x() - pb.x());
        let length = f64::sqrt((pa.x() - pb.x()).powi(2) + (pa.y() - pb.y()));

        let r = f64::sqrt(f64::min(pa.magnitude2(), pb.magnitude2()));

        Segment {
            r,
            theta,
            r_max: r + length,
        }
    }

    fn contains(&self, p: &impl Coord) -> bool {
        let p_theta = f64::atan2(p.y(), p.x());

        if almost_eq(self.theta, p_theta, 1e-8) {
            let p_r = p.magnitude2();

            if self.r.powi(2) >= p_r && p_r <= self.r_max.powi(2) {
                return true;
            }
        }

        return false;
    }

    fn intersects(&self, other: &Segment) -> Option<Point> {
        // same abs theta means parallel lines
        if self.theta.abs() == other.theta.abs() {
            return None;
        }

        todo!()
    }
}

fn straight_line(pa: &Point, pb: &Point) -> (f64, f64) {
    // straight line of the form y = mx + p
    // really hoping we will not get vertical lines here or else this might break
    let m = (pa.y - pb.y) / (pa.x - pb.x);
    let p = pa.y - m * pa.x;
    return (m, p);
}

fn line_intersect(l1: (f64, f64), l2: (f64, f64)) -> Option<Point> {
    let (a, b) = l1;
    let (c, d) = l2;

    if a == c {
        // parallel lines yield no intersection
        return None;
    }

    let x = (d - b) / (a - c);
    let y = a * x + b;

    return Some(Point { x, y });
}

fn on_segment(point: &Point, start: &Point, end: &Point) -> bool {
    let (x_max, x_min) = max_min(start.x, end.x);

    if point.x >= x_min && point.x <= x_max {
        let (y_max, y_min) = max_min(start.y, end.y);

        if point.y >= y_min && point.y <= y_max {
            return true;
        }
    }

    return false;
}

fn is_inside(poly: &Polygon<Point>, other: &Polygon<Point>) -> Result<(), Vec<bool>> {
    let mut all_inside = true;
    let mut inside: Vec<bool> = Vec::with_capacity(poly.points().len());

    for p in poly.points() {
        let mut cross_count = 0;

        for i in 0..other.points().len() {
            let j = if i == 0 {
                other.points().len() - 1
            } else {
                i - 1
            };

            let start = &other.points()[j];
            let end = &other.points()[i];

            let epsilon = 1e-12;

            if almost_eq(start.x, end.x, epsilon) {
                if p.x < start.x {
                    let (y_max, y_min) = max_min(start.y, end.y);

                    if p.y >= y_min && p.x <= y_max {
                        cross_count += 1;
                    }
                }
                continue;
            } else if almost_eq(start.y, end.y, epsilon) {
                //ignore tangent lines
                continue;
            }

            // derived from y = ax + b
            let a = (start.y - end.y) / (start.x - end.x);
            let b = start.y - a * start.x;
            let cross_x = (p.y - b) / a;

            if cross_x >= p.x {
                let (max_x, min_x) = max_min(start.x, end.x);

                if cross_x >= min_x && cross_x <= max_x {
                    cross_count += 1
                }
            }
        }

        if cross_count % 2 == 0 {
            // crossing an even number of edges means we are outside the polygon
            all_inside = false;
            inside.push(false);
        } else {
            inside.push(true);
        }
    }

    if all_inside {
        return Ok(());
    } else {
        return Err(inside);
    };
}

fn almost_eq(a: f64, b: f64, epsilon: f64) -> bool {
    f64::abs(a - b) <= epsilon
}

fn max_min(a: f64, b: f64) -> (f64, f64) {
    if a > b {
        (a, b)
    } else {
        (b, a)
    }
}

#[cfg(test)]
mod test {

    use voronator::{delaunator::Point, polygon::Polygon};

    use super::*;
    #[test]
    fn triangle_inside_square() {
        let square = Polygon::from_points(vec![
            Point { x: -1.0, y: -1.0 },
            Point { x: -1.0, y: 1.0 },
            Point { x: 1.0, y: 1.0 },
            Point { x: 1.0, y: -1.0 },
        ]);

        let triangle = Polygon::from_points(vec![
            Point { x: 0.0, y: 0.5 },
            Point { x: -0.5, y: -0.5 },
            Point { x: 0.5, y: -0.5 },
        ]);

        assert!(is_inside(&triangle, &square).is_ok())
    }

    #[test]
    fn triangle_inside_triangle() {
        let t1 = Polygon::from_points(vec![
            Point { x: 0.0, y: 1.0 },
            Point { x: -1.0, y: -1.0 },
            Point { x: 1.0, y: -1.0 },
        ]);

        let t2 = Polygon::from_points(vec![
            Point { x: 0.0, y: 0.5 },
            Point { x: -0.5, y: -0.5 },
            Point { x: 0.5, y: -0.5 },
        ]);

        assert!(is_inside(&t2, &t1).is_ok())
    }

    #[test]
    fn nothing() {
        let t1 = Polygon::from_points(vec![
            Point { x: 0.0, y: 1.0 },
            Point { x: -1.0, y: -1.0 },
            Point { x: 1.0, y: -1.0 },
        ]);

        let _ = regularize(&t1, &t1, &vec![false, false, true]);

        assert!(false)
    }
}
