use super::*;
use crate::*;

use std::fs::File;

#[test]
fn check_test_files_exist() -> std::io::Result<()> {
    File::open("euc_2d")?;
    File::open("lower_diag_row")?;
    File::open("full_matrix")?;

    Ok(())
}

fn check_file_type_works(filename: &str, file_type: TspFileType) {
    let file_content = std::fs::read_to_string(filename).expect("file doesn't exist");
    let mut lines = file_content.lines();

    lines.nth(3);

    let tsp_type = TspParser::check_file_type(&mut lines).expect("file couldn't be parsed");
    assert_eq!(file_type, tsp_type);
}

#[test]
fn check_euc_2d_file_type_works() {
    check_file_type_works("euc_2d", TspFileType::Euc2d);
}

#[test]
fn check_full_matrix_file_type_works() {
    check_file_type_works("full_matrix", TspFileType::FullMatrix);
}

#[test]
fn check_lower_diag_row_file_type_works() {
    check_file_type_works("lower_diag_row", TspFileType::LowerDiagRow);
}

fn check_dimension_works(filename: &str, expected_dimension: usize) {
    let file_content = std::fs::read_to_string(filename).expect("file doesn't exist");
    let mut lines = file_content.lines();

    let dimension = TspParser::check_dimension(&mut lines).expect("file couldn't be parsed");
    assert_eq!(expected_dimension, dimension);
}

#[test]
fn check_euc_2d_dimension_works() {
    check_dimension_works("euc_2d", 3);
}

#[test]
fn check_full_matrix_dimension_works() {
    check_dimension_works("full_matrix", 3);
}

#[test]
fn check_lower_diag_row_dimension_works() {
    check_dimension_works("lower_diag_row", 3);
}

#[test]
fn lower_diag_row_works() {
    let tsp = TspParser::from_file("lower_diag_row").expect("Couldn't parse file");
    assert_eq!(vec![vec![0, 2, 3], vec![2, 0, 3], vec![3, 3, 0]], tsp.get_edges());
}

#[test]
fn full_matrix_works() {
    let tsp = TspParser::from_file("full_matrix").expect("Couldn't parse file");
    assert_eq!(
        vec![vec![9999, 2, 3], vec![2, 9999, 3], vec![3, 3, 9999]],
        tsp.get_edges()
    );
}

#[test]
fn euc_2d_works() {
    let tsp = TspParser::from_file("euc_2d").expect("Couldn't parse file");
    assert_eq!(
        vec![vec![0, 10, 7], vec![10, 0, 7], vec![7, 7, 0]],
        tsp.get_edges()
    );
}

#[test]
fn route_len_works() {
    let tsp = TspParser::from_file("full_matrix").expect("Couldn't parse file");
    let route = [0, 1, 2];

    let route_len = tsp.get_route_len(&route).expect("route should be valid");

    assert_eq!(8, route_len);
}

#[test]
fn route_len_too_short() {
    let tsp = TspParser::from_file("full_matrix").expect("Couldn't parse file");
    let route = [0, 1];

    let route_len = tsp.get_route_len(&route);

    assert!(route_len.is_err());
}

#[test]
fn route_len_not_permutation() {
    let tsp = TspParser::from_file("full_matrix").expect("Couldn't parse file");
    let route = [0, 1, 1];

    let route_len = tsp.get_route_len(&route);

    assert!(route_len.is_err());
}

#[test]
fn krandom_works() {
    let tsp = TspParser::from_file("full_matrix").expect("Couldn't parse test file");
    let route = KRandom::new(10).get_route(&tsp);
    let route_len = tsp.get_route_len(&route).expect("Has to be valid route");

    assert!(route_len > 0);
}

#[test]
fn nearest_neighbour_works() {
    let tsp = TspParser::from_file("full_matrix").expect("Couldn't parse test file");
    let route = NearestNeighbour::new().get_route(&tsp);
    let route_len = tsp.get_route_len(&route).expect("Has to be valid route.");

    assert!(route_len > 0);
}

#[test]
fn nearest_neighbour_optimized_works() {
    let tsp = TspParser::from_file("full_matrix").expect("Couldn't parse test file");
    let route = NearestNeighbourOptimized::new().get_route(&tsp);
    let route_len = tsp.get_route_len(&route).expect("Has to be valid route.");

    assert!(route_len > 0);
}

#[test]
fn two_opt_works() {
    let tsp = TspParser::from_file("full_matrix").expect("Couldn't parse test file");
    let heuristic = NearestNeighbourOptimized::new();
    let route = TwoOpt::new(Box::new(heuristic)).get_route(&tsp);
    let route_len = tsp.get_route_len(&route).expect("Has to be valid route.");

    assert!(route_len > 0);
}
