use std::str::Lines;

use crate::tsp::{Tsp, TspType};

mod euc2d;
mod full_matrix;
mod lower_diag_row;

#[cfg(test)]
mod tests;

use euc2d::Euc2dTspParser;
use full_matrix::FullMatrixTspParser;
use lower_diag_row::LowerDiagRowTspParser;


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TspFileType {
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

pub trait VariantParser {
    fn parse(file_lines: &mut Lines, dimension: usize) -> Result<Vec<Vec<u32>>, TspParsingError>;
}

pub struct TspParser;

impl TspParser {
    pub fn from_file(filename: &str) -> Result<Tsp, TspParsingError> {
        let file_content =
            std::fs::read_to_string(filename).map_err(|_| TspParsingError::FileDoesntExist)?;
        let mut file_lines = file_content.lines();

        let dimension = TspParser::check_dimension(&mut file_lines)?;
        let file_type = TspParser::check_file_type(&mut file_lines)?;

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

        match file_type {
            TspFileType::FullMatrix => Ok(Tsp::new(edges, dimension, TspType::Asymmetric)),
            _ => Ok(Tsp::new(edges, dimension, TspType::Symmetric)),
        }
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
            TspParser::check_explicit_file_type(file_lines)
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
