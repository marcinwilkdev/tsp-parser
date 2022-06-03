use crate::tsp::Tsp;

mod krandom;
mod nearest_neighbour;
mod two_opt;

pub use krandom::KRandom;
pub use nearest_neighbour::{NearestNeighbour, NearestNeighbourOptimized};
pub use two_opt::TwoOpt;
pub use two_opt::best_neighbourhood_invert;

pub trait TspHeuristic {
    fn get_route(&self, tsp: &Tsp) -> Vec<usize>;
}
