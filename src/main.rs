use tsp_parser::*;

pub const K: usize = 1000;
// pub const FILE: &'static str = "test_files/fl1400.tsp";
pub const FILE: &'static str = "test_files/berlin52.tsp";

fn main() {
    println!("File: {}", FILE);

    let start = std::time::Instant::now();

    let tsp = TspParser::from_file(FILE).expect("Couldn't parse test file");

    println!("Time to load problem: {:?}\n", std::time::Instant::now() - start);

    let start = std::time::Instant::now();

    let route = KRandom::new(K).get_route(&tsp);
    let route_len = tsp.get_route_len(&route).expect("Has to be valid route");

    println!("K-random (k = {}) route len: {}", K, route_len);
    println!("Time to calculate k-random: {:?}\n", std::time::Instant::now() - start);

    let start = std::time::Instant::now();

    let route = NearestNeighbour::new().get_route(&tsp);
    let route_len = tsp.get_route_len(&route).expect("Has to be valid route");

    println!("Nearest neighbour route len: {}", route_len);
    println!("Time to calculate nearest neighbour: {:?}\n", std::time::Instant::now() - start);

    let start = std::time::Instant::now();

    let route = NearestNeighbourOptimized::new().get_route(&tsp);
    let route_len = tsp.get_route_len(&route).expect("Has to be valid route");

    println!("Nearest neighbour optimized route len: {}", route_len);
    println!("Time to calculate nearest neighbour optimized: {:?}\n", std::time::Instant::now() - start);

    let start = std::time::Instant::now();

    let heuristic = NearestNeighbourOptimized::new();
    let route = TwoOpt::new(Box::new(heuristic)).get_route(&tsp);
    let route_len = tsp.get_route_len(&route).expect("Has to be valid route");

    println!("Two opt route len: {}", route_len);
    println!("Time to calculate two opt: {:?}\n", std::time::Instant::now() - start);
}
