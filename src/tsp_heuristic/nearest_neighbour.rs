use rand::prelude::*;

use crate::{Tsp, TspHeuristic};

fn nearest_neighbour_inner(tsp: &Tsp, starting_vertex: usize) -> Vec<usize> {
    let dimension = tsp.get_dimension();

    assert!(starting_vertex < dimension, "Vertex bigger than dimension");

    let mut route = vec![0; dimension];
    let mut visited = vec![false; dimension];

    route[0] = starting_vertex;
    visited[starting_vertex] = true;

    let mut curr_vertex = starting_vertex;

    for i in 1..dimension {
        let mut min_len = u32::MAX;
        let mut next_vertex = 0;

        for i in 0..dimension {
            if visited[i] {
                continue;
            }

            let curr_len = tsp.get_edges()[curr_vertex][i];

            if curr_len < min_len {
                min_len = curr_len;
                next_vertex = i;
            }
        }

        visited[next_vertex] = true;
        route[i] = next_vertex;
        curr_vertex = next_vertex;
    }

    route
}

pub struct NearestNeighbour;

impl NearestNeighbour {
    pub fn new() -> NearestNeighbour {
        NearestNeighbour
    }
}

impl TspHeuristic for NearestNeighbour {
    fn get_route(&self, tsp: &Tsp) -> Vec<usize> {
        let first_vertex = thread_rng().gen_range(0..tsp.get_dimension());

        nearest_neighbour_inner(tsp, first_vertex)
    }
}

pub struct NearestNeighbourOptimized;

impl NearestNeighbourOptimized {
    pub fn new() -> NearestNeighbourOptimized {
        NearestNeighbourOptimized
    }
}

impl TspHeuristic for NearestNeighbourOptimized {
    fn get_route(&self, tsp: &Tsp) -> Vec<usize> {
        let mut best_route = None;
        let mut best_route_len = None;

        for i in 0..tsp.get_dimension() {
            let route = nearest_neighbour_inner(tsp, i);
            let route_len = tsp.get_route_len(&route).expect("has to be valid route");

            if (best_route.is_none() && best_route_len.is_none())
                || route_len < best_route_len.unwrap()
            {
                best_route = Some(route);
                best_route_len = Some(route_len);
            }
        }

        best_route.expect("has to be valid route")
    }
}
