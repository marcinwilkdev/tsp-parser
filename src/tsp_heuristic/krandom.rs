use rand::prelude::*;

use crate::{TspHeuristic, Tsp};

pub struct KRandom {
    k: usize,
}

impl KRandom {
    pub fn new(k: usize) -> KRandom {
        KRandom { k }
    }
}

impl TspHeuristic for KRandom {
    fn get_route(&self, tsp: &Tsp) -> Vec<usize> {
        let mut route = (0..tsp.get_dimension()).collect::<Vec<_>>();
        let mut best_route = None;
        let mut best_route_len = None;
        let mut rng = rand_pcg::Pcg64Mcg::new(thread_rng().gen());

        for _ in 0..self.k {
            route.shuffle(&mut rng);
            let route_len = tsp.get_route_len(&route).expect("has to be valid route");

            if (best_route.is_none() && best_route_len.is_none())
                || route_len < best_route_len.unwrap()
            {
                best_route = Some(route.clone());
                best_route_len = Some(route_len);
            }
        }

        best_route.expect("there has to be some route")
    }
}
