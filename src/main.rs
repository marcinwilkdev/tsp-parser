use tsp_parser::*;

pub const K: usize = 1000;
pub const FILE: &'static str = "test_files/d1655.tsp";

fn main() {
    println!("File: {}", FILE);

    let tsp = get_problem_with_bench(FILE);

    let HeuristicBench {
        route: _,
        route_len,
        duration,
    } = run_heuristic_with_bench(&tsp, KRandom::new(K));

    println!("K-random (k = {}) route len: {}", K, route_len);
    println!("Time to calculate k-random: {:?}\n", duration);

    let HeuristicBench {
        route: _,
        route_len,
        duration,
    } = run_heuristic_with_bench(&tsp, NearestNeighbour::new());

    println!("Nearest neighbour route len: {}", route_len);
    println!("Time to calculate nearest neighbour: {:?}\n", duration);

    let HeuristicBench {
        route: _,
        route_len,
        duration,
    } = run_heuristic_with_bench(&tsp, NearestNeighbourOptimized::new());

    println!("Nearest neighbour optimized route len: {}", route_len);
    println!("Time to calculate nearest neighbour opt: {:?}\n", duration);

    // at this moment only invert neighbourhood is turned on
    // works properly only for symetric tsp
    let HeuristicBench {
        route: _,
        route_len,
        duration,
    } = run_heuristic_with_bench(&tsp, TwoOpt::new(NearestNeighbourOptimized::new()));

    println!("Two opt route len: {}", route_len);
    println!("Time to calculate two opt: {:?}\n", duration);
}
