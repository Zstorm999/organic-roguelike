pub mod voronoi;

use voronoice::Point;

use rand::{distributions::Uniform, thread_rng, Rng};

pub fn random_points(number: usize) -> Vec<Point> {
    let distr = Uniform::new(-1.0, 1.0);

    thread_rng()
        .sample_iter(distr)
        .zip(thread_rng().sample_iter(distr))
        .map(|(x, y)| Point { x, y })
        .take(number)
        .collect()
}
