pub mod neighbourhood;
mod tsp;
mod tsp_heuristic;
mod tsp_parser;
mod tsp_tests;

pub use crate::tsp::*;
pub use crate::tsp_heuristic::*;
pub use crate::tsp_parser::{TspParser, TspFileType};

pub struct HeuristicBench {
    pub route: Vec<usize>,
    pub route_len: u32,
    pub duration: std::time::Duration,
}

pub fn get_problem_with_bench(file: &str) -> Tsp {
    let start = std::time::Instant::now();

    let tsp = TspParser::from_file(file).expect("Couldn't parse test file");

    println!(
        "Time to load problem: {:?}\n",
        std::time::Instant::now() - start
    );

    tsp
}

pub fn run_heuristic_with_bench<H>(tsp: &Tsp, heuristic: H) -> HeuristicBench
where
    H: TspHeuristic,
{
    let start = std::time::Instant::now();

    let route = heuristic.get_route(&tsp);
    let route_len = tsp.get_route_len(&route).expect("Has to be valid route");

    HeuristicBench {
        route,
        route_len,
        duration: std::time::Instant::now() - start,
    }
}
