use level_generator::{
    self,
    voronoi::{self, generate_voronoi},
    Point,
};

use graplot::Scatter;

fn main() {
    let points = level_generator::random_points(20);

    let x_pos: Vec<_> = points.iter().map(|p| p.x).collect();
    let y_pos: Vec<_> = points.iter().map(|p| p.y).collect();

    let mut plot = Scatter::new((x_pos, y_pos));

    let alpha_shape = get_alpha_shape(&points);
    plot.add((alpha_shape.0, alpha_shape.1, "r-o"));

    let cells = generate_voronoi(points);

    for c in cells.iter_cells() {
        let x_pos: Vec<_> = c
            .points()
            .iter()
            .cycle()
            .take(c.points().len() + 1)
            .map(|p| p.x)
            .collect();
        let y_pos: Vec<_> = c
            .points()
            .iter()
            .cycle()
            .take(c.points().len() + 1)
            .map(|p| p.y)
            .collect();

        plot.add((x_pos, y_pos, "b-o"))
    }

    plot.show();
}

fn get_alpha_shape(points: &[Point]) -> (Vec<f64>, Vec<f64>) {
    let polygon = voronoi::alpha_shape(&points, f64::INFINITY);

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

    let x_pos = polygon
        .points()
        .iter()
        .cycle()
        .take(polygon.points().len() + 1)
        .map(|p| p.x)
        .collect();
    let y_pos = polygon
        .points()
        .iter()
        .cycle()
        .take(polygon.points().len() + 1)
        .map(|p| p.y)
        .collect();

    (x_pos, y_pos)
}
