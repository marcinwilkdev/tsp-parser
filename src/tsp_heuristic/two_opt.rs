use crate::{Tsp, TspHeuristic};

pub struct TwoOpt {
    initial_heuristic: Box<dyn TspHeuristic>,
}

impl TwoOpt {
    pub fn new(initial_heuristic: Box<dyn TspHeuristic>) -> TwoOpt {
        TwoOpt { initial_heuristic }
    }
}

fn best_neighbourhood<F>(tsp: &Tsp, route: Vec<usize>, neighbourhood: F) -> Vec<usize>
where
    F: Fn(&mut [usize]),
{
    let dimension = tsp.get_dimension();

    let mut best_route = route;
    let mut best_route_len = tsp
        .get_route_len(&best_route)
        .expect("has to be valid route");

    let mut indexes = None;
    let mut curr_best_route_len = None;

    loop {
        for i in 0..dimension {
            for j in i..dimension {
                neighbourhood(&mut best_route[i..=j]);
                let route_len = tsp.get_route_len(&best_route).expect("has to be some");

                if (curr_best_route_len.is_none() && route_len < best_route_len)
                    || (curr_best_route_len.is_some() && route_len < curr_best_route_len.unwrap())
                {
                    indexes = Some((i, j));
                    curr_best_route_len = Some(route_len);
                }

                neighbourhood(&mut best_route[i..=j]);
            }
        }

        if curr_best_route_len.is_none() && indexes.is_none() {
            break;
        }

        let (i, j) = indexes.unwrap();

        neighbourhood(&mut best_route[i..=j]);
        best_route_len = curr_best_route_len.unwrap();

        indexes = None;
        curr_best_route_len = None;
    }

    best_route
}

fn invert(route: &mut [usize]) {
    let route_len = route.len();
    let half_route_len = route_len / 2;

    for i in 0..=half_route_len {
        route.swap(i, route_len - i - 1);
    }
}

// fn swap(route: &mut [usize]) {
//     route.swap(0, route.len() - 1)
// }

impl TspHeuristic for TwoOpt {
    fn get_route(&self, tsp: &Tsp) -> Vec<usize> {
        let initial_route = self.initial_heuristic.get_route(tsp);
        best_neighbourhood(tsp, initial_route, invert)
    }
}
