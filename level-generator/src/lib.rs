pub mod voronoi;

pub use voronator::delaunator::Point;
pub use voronator::polygon::Polygon;

use rand::{distributions::Uniform, rngs::SmallRng, Rng, SeedableRng};

pub fn random_points(number: usize) -> Vec<Point> {
    let distr = Uniform::new(-1.0, 1.0);

    let gen1 = SmallRng::seed_from_u64(150);
    let gen2 = SmallRng::seed_from_u64(42);

    gen1.sample_iter(distr)
        .zip(gen2.sample_iter(distr))
        .map(|(x, y)| Point { x, y })
        .take(number)
        .collect()
}
