use std::str::Lines;

use crate::tsp_parser::{VariantParser, TspParsingError};

pub struct Euc2dTspParser;

impl VariantParser for Euc2dTspParser {
    fn parse(file_lines: &mut Lines, dimension: usize) -> Result<Vec<Vec<u32>>, TspParsingError> {
        let coords: Result<Vec<(i32, i32)>, TspParsingError> = file_lines
            .take(dimension)
            .map(|line| Euc2dTspParser::parse_line_into_coords(&line))
            .collect();

        let edges = Euc2dTspParser::parse_distances(&coords?);

        Ok(edges)
    }
}

impl Euc2dTspParser {
    fn parse_line_into_coords(line: &str) -> Result<(i32, i32), TspParsingError> {
        let mut line = line.split_whitespace();

        let x = line.nth(1).ok_or(TspParsingError::NotEnoughData)?;
        let y = line.next().ok_or(TspParsingError::NotEnoughData)?;

        let x: f64 = x.parse().map_err(|_| TspParsingError::WeightNotANumber)?;
        let y: f64 = y.parse().map_err(|_| TspParsingError::WeightNotANumber)?;

        Ok((x.round() as i32, y.round() as i32))
    }

    fn parse_distances(coords: &[(i32, i32)]) -> Vec<Vec<u32>> {
        coords
            .iter()
            .map(|p1| Euc2dTspParser::calculate_distances_to_other_points(*p1, coords))
            .collect()
    }

    fn calculate_distances_to_other_points(p1: (i32, i32), coords: &[(i32, i32)]) -> Vec<u32> {
        coords
            .iter()
            .map(|p2| Euc2dTspParser::calculate_distance(p1, *p2))
            .collect()
    }

    fn calculate_distance((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> u32 {
        (((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)) as f64)
            .sqrt()
            .round() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn euc2d_parser_working() {
        let data = "1 0.0 10.0
2 0.0 0.0
3 5.0 5.0";

        let mut data_lines = data.lines();

        let edges = Euc2dTspParser::parse(&mut data_lines, 3).expect("error while parsing data");

        assert_eq!(
            vec![vec![0, 10, 7], vec![10, 0, 7], vec![7, 7, 0]],
            edges
        );
    }
}
