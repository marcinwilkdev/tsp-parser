use std::str::Lines;

use crate::tsp_parser::{VariantParser, TspParsingError};

pub struct FullMatrixTspParser;

impl VariantParser for FullMatrixTspParser {
    fn parse(file_lines: &mut Lines, dimension: usize) -> Result<Vec<Vec<u32>>, TspParsingError> {
        let mut edges = Vec::new();

        let mut curr_line = file_lines.next().ok_or(TspParsingError::NotEnoughData)?;

        let mut line_weights = curr_line.split_whitespace();

        for _ in 0..dimension {
            let mut curr_edges = Vec::new();

            let mut edge_counter = 0;

            while edge_counter < dimension {
                match line_weights.next() {
                    Some(edge) => {
                        let edge = edge
                            .parse()
                            .map_err(|_| TspParsingError::WeightNotANumber)?;

                        curr_edges.push(edge);
                        edge_counter += 1;
                    }
                    None => {
                        curr_line = file_lines.next().ok_or(TspParsingError::NotEnoughData)?;
                        line_weights = curr_line.split_whitespace();
                    }
                }
            }

            edges.push(curr_edges);
        }

        Ok(edges)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_matrix_parser_working() {
        let data = "9999    2    3
    2 9999    3
    3    3 9999";

        let mut data_lines = data.lines();

        let edges =
            FullMatrixTspParser::parse(&mut data_lines, 3).expect("error while parsing data");
        assert_eq!(
            vec![vec![9999, 2, 3], vec![2, 9999, 3], vec![3, 3, 9999]],
            edges
        );
    }
}
