use voronoice::{BoundingBox, Point, Voronoi, VoronoiBuilder, VoronoiCell};

pub struct Cells(Voronoi);

pub struct Cell<'a>(VoronoiCell<'a>);

impl Cells {
    pub fn iter_cells(&self) -> impl Iterator<Item = Cell> + Clone {
        self.0.iter_cells().map(|c| Cell(c))
    }
}

impl Cell<'_> {
    pub fn iter_vertices(&self) -> impl Iterator<Item = [f32; 2]> + Clone + '_ {
        self.0.iter_vertices().map(|v| [v.x as f32, -v.y as f32])
    }
}

pub fn generate_voronoi(points: Vec<Point>) -> Cells {
    Cells(
        VoronoiBuilder::default()
            .set_sites(points)
            .set_bounding_box(BoundingBox::new_centered_square(2.0))
            .build()
            .unwrap(),
    )
}
