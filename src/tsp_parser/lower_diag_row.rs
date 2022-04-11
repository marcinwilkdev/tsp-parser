use std::str::Lines;

use crate::tsp_parser::{VariantParser, TspParsingError};

pub struct LowerDiagRowTspParser;

impl VariantParser for LowerDiagRowTspParser {
    fn parse(file_lines: &mut Lines, dimension: usize) -> Result<Vec<Vec<u32>>, TspParsingError> {
        let data_lines = file_lines.filter(|line| !(line == &"EOF"));

        let edges_collector = EdgesCollector::new(dimension);

        let edges = edges_collector.collect_edges(data_lines)?;

        Ok(edges)
    }
}

struct EdgesCollector {
    edges: Vec<Vec<u32>>,
    curr_edge: Vec<u32>,
    edge_index: usize,
    dimension: usize,
}

impl EdgesCollector {
    fn new(dimension: usize) -> Self {
        EdgesCollector {
            edges: Vec::new(),
            curr_edge: Vec::new(),
            edge_index: 0,
            dimension,
        }
    }

    // TODO: refactor this into smaller pieces
    fn collect_edges<'a, I>(mut self, data_lines: I) -> Result<Vec<Vec<u32>>, TspParsingError>
    where
        I: Iterator<Item = &'a str>,
    {
        let mut line_weight_index = 0;

        for line in data_lines {
            let line_weights = line.split_whitespace();

            for weight in line_weights {
                let weight = weight
                    .parse()
                    .map_err(|_| TspParsingError::WeightNotANumber)?;

                self.curr_edge.push(weight);

                if weight != 0 {
                    self.edges[self.edge_index].push(weight);
                    self.edge_index += 1;
                } else {
                    self.edges.push(self.curr_edge);
                    self.curr_edge = Vec::new();
                    self.edge_index = 0;

                    line_weight_index += 1;

                    if line_weight_index == self.dimension {
                        return Ok(self.edges);
                    }
                }
            }
        }

        Ok(self.edges)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lower_diag_row_parser_works() {
        let data = "0 2 0 3
 3 0
EOF";

        let mut data_lines = data.lines();

        let edges =
            LowerDiagRowTspParser::parse(&mut data_lines, 3).expect("error while parsing data");
        assert_eq!(vec![vec![0, 2, 3], vec![2, 0, 3], vec![3, 3, 0]], edges);
    }
}
