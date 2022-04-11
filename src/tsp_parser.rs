use std::str::Lines;

use rand::prelude::*;

mod euc2d;
mod full_matrix;
mod lower_diag_row;

use euc2d::Euc2dTspParser;
use full_matrix::FullMatrixTspParser;
use lower_diag_row::LowerDiagRowTspParser;

#[derive(Debug)]
pub struct Tsp {
    edges: Vec<Vec<u32>>,
    dimension: usize,
}

#[derive(Debug, PartialEq)]
enum TspFileType {
    LowerDiagRow,
    FullMatrix,
    Euc2d,
}

#[derive(Debug)]
pub enum TspParsingError {
    FileDoesntExist,
    NoExplicitFileType,
    NoFileType,
    NoDimension,
    NotEnoughData,
    DimensionNotANumber,
    WeightNotANumber,
}

#[derive(Debug)]
pub enum TspRouteError {
    TooShort,
    NotPermutation,
}

pub trait TspParser {
    fn parse(file_lines: &mut Lines, dimension: usize) -> Result<Vec<Vec<u32>>, TspParsingError>;
}

impl Tsp {
    pub fn from_file(filename: &str) -> Result<Tsp, TspParsingError> {
        let file_content =
            std::fs::read_to_string(filename).map_err(|_| TspParsingError::FileDoesntExist)?;
        let mut file_lines = file_content.lines();

        let dimension = Tsp::check_dimension(&mut file_lines)?;
        let file_type = Tsp::check_file_type(&mut file_lines)?;

        loop {
            let line = file_lines.next().ok_or(TspParsingError::NotEnoughData)?;

            if line.contains("EDGE_WEIGHT_SECTION") || line.contains("NODE_COORD_SECTION") {
                break;
            }
        }

        let edges = match file_type {
            TspFileType::LowerDiagRow => LowerDiagRowTspParser::parse(&mut file_lines, dimension),
            TspFileType::FullMatrix => FullMatrixTspParser::parse(&mut file_lines, dimension),
            TspFileType::Euc2d => Euc2dTspParser::parse(&mut file_lines, dimension),
        }?;

        Ok(Tsp { edges, dimension })
    }

    pub fn get_route_len(&self, route: &[usize]) -> Result<u32, TspRouteError> {
        if route.len() != self.dimension {
            return Err(TspRouteError::TooShort);
        }

        let mut route_clone = route.to_vec();

        route_clone.sort_unstable();

        if route_clone != (0..self.dimension).collect::<Vec<_>>() {
            return Err(TspRouteError::NotPermutation);
        }

        let mut route_len = 0;

        for i in 0..self.dimension - 1 {
            let first_vertex = route[i];
            let second_vertex = route[i + 1];

            route_len += self.edges[first_vertex][second_vertex]
        }

        let first_vertex = route[0];
        let last_vertex = route[route.len() - 1];

        route_len += self.edges[first_vertex][last_vertex];

        Ok(route_len)
    }

    pub fn get_edges(self) -> Vec<Vec<u32>> {
        self.edges
    }

    pub fn krandom_route(&self, k: usize) -> Vec<usize> {
        let route = (0..self.dimension).collect::<Vec<_>>();
        let mut best_route = None;
        let mut best_route_len = None;
        let mut rng = rand_pcg::Pcg64Mcg::new(thread_rng().gen());

        for _ in 0..k {
            let mut route_clone = route.clone();
            route_clone.shuffle(&mut rng);

            let route_clone_len = self.get_route_len(&route_clone).expect("has to be valid route");

            if (best_route.is_none() && best_route_len.is_none()) || route_clone_len < best_route_len.unwrap() {
                best_route = Some(route_clone);
                best_route_len = Some(route_clone_len);
            }
        }

        best_route.expect("there has to be some route")
    }

    fn check_dimension(file_lines: &mut Lines) -> Result<usize, TspParsingError> {
        let dimension = loop {
            let line = file_lines.next().ok_or(TspParsingError::NoDimension)?;

            if line.contains("DIMENSION") {
                break line;
            }
        };

        dimension
            .split_whitespace()
            .last()
            .ok_or(TspParsingError::NoDimension)?
            .parse::<usize>()
            .map_err(|_| TspParsingError::DimensionNotANumber)
    }

    fn check_file_type(file_lines: &mut Lines) -> Result<TspFileType, TspParsingError> {
        let edge_weight_type = loop {
            let line = file_lines.next().ok_or(TspParsingError::NoFileType)?;

            if line.contains("EDGE_WEIGHT_TYPE") {
                break line;
            }
        };

        if edge_weight_type.contains("EUC_2D") {
            Ok(TspFileType::Euc2d)
        } else if edge_weight_type.contains("EXPLICIT") {
            Tsp::check_explicit_file_type(file_lines)
        } else {
            Err(TspParsingError::NoFileType)
        }
    }

    fn check_explicit_file_type(file_lines: &mut Lines) -> Result<TspFileType, TspParsingError> {
        let edge_weight_format = file_lines
            .next()
            .ok_or(TspParsingError::NoExplicitFileType)?;

        if edge_weight_format.contains("FULL_MATRIX") {
            Ok(TspFileType::FullMatrix)
        } else if edge_weight_format.contains("LOWER_DIAG_ROW") {
            Ok(TspFileType::LowerDiagRow)
        } else {
            Err(TspParsingError::NoExplicitFileType)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let tsp_type = Tsp::check_file_type(&mut lines).expect("file couldn't be parsed");
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

        let dimension = Tsp::check_dimension(&mut lines).expect("file couldn't be parsed");
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
        let tsp = Tsp::from_file("lower_diag_row").expect("Couldn't parse file");
        assert_eq!(vec![vec![0, 2, 3], vec![2, 0, 3], vec![3, 3, 0]], tsp.edges);
    }

    #[test]
    fn full_matrix_works() {
        let tsp = Tsp::from_file("full_matrix").expect("Couldn't parse file");
        assert_eq!(
            vec![vec![9999, 2, 3], vec![2, 9999, 3], vec![3, 3, 9999]],
            tsp.edges
        );
    }

    #[test]
    fn euc_2d_works() {
        let tsp = Tsp::from_file("euc_2d").expect("Couldn't parse file");
        assert_eq!(
            vec![vec![0, 10, 7], vec![10, 0, 7], vec![7, 7, 0]],
            tsp.edges
        );
    }

    #[test]
    fn route_len_works() {
        let tsp = Tsp::from_file("full_matrix").expect("Couldn't parse file");
        let route = [0, 1, 2];

        let route_len = tsp.get_route_len(&route).expect("route should be valid");

        assert_eq!(8, route_len);
    }

    #[test]
    fn route_len_too_short() {
        let tsp = Tsp::from_file("full_matrix").expect("Couldn't parse file");
        let route = [0, 1];

        let route_len = tsp.get_route_len(&route);

        assert!(route_len.is_err());
    }

    #[test]
    fn route_len_not_permutation() {
        let tsp = Tsp::from_file("full_matrix").expect("Couldn't parse file");
        let route = [0, 1, 1];

        let route_len = tsp.get_route_len(&route);

        assert!(route_len.is_err());
    }

    #[test]
    fn krandom_works() {
        let tsp = Tsp::from_file("full_matrix").expect("Couldn't parse test file");
        let route = tsp.krandom_route(10);
        let route_len = tsp.get_route_len(&route).expect("Has to be valid route");

        assert!(route_len > 0);
    }
}
