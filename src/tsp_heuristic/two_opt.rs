use crate::{neighbourhood, Tsp, TspHeuristic};

pub struct TwoOpt<H>
where
    H: TspHeuristic,
{
    initial_heuristic: H,
}

impl<H> TwoOpt<H>
where
    H: TspHeuristic,
{
    pub fn new(initial_heuristic: H) -> Self {
        TwoOpt { initial_heuristic }
    }
}

fn best_neighbourhood_generic<F>(tsp: &Tsp, route: Vec<usize>, neighbourhood: F) -> Vec<usize>
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
                neighbourhood(&mut best_route[i..=j]);

                if (curr_best_route_len.is_none() && route_len < best_route_len)
                    || (curr_best_route_len.is_some() && route_len < curr_best_route_len.unwrap())
                {
                    indexes = Some((i, j));
                    curr_best_route_len = Some(route_len);
                }
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

fn best_neighbourhood_invert(tsp: &Tsp, route: Vec<usize>) -> Vec<usize> {
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
                let route_len = tsp.get_inverted_route_len(&best_route, best_route_len, i, j);

                if (curr_best_route_len.is_none() && route_len < best_route_len)
                    || (curr_best_route_len.is_some() && route_len < curr_best_route_len.unwrap())
                {
                    indexes = Some((i, j));
                    curr_best_route_len = Some(route_len);
                }
            }
        }

        if curr_best_route_len.is_none() && indexes.is_none() {
            break;
        }

        let (i, j) = indexes.unwrap();

        neighbourhood::invert(&mut best_route[i..=j]);
        best_route_len = curr_best_route_len.unwrap();

        indexes = None;
        curr_best_route_len = None;
    }

    best_route
}

impl<H> TspHeuristic for TwoOpt<H>
where
    H: TspHeuristic,
{
    fn get_route(&self, tsp: &Tsp) -> Vec<usize> {
        let initial_route = self.initial_heuristic.get_route(tsp);
        best_neighbourhood_invert(tsp, initial_route)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_route_invert_len() {
        let tsp = get_problem_with_bench("test_files/berlin52.tsp");

        let HeuristicBench {
            mut route,
            route_len,
            duration: _,
        } = run_heuristic_with_bench(&tsp, NearestNeighbourOptimized::new());

        let i = 0;
        let j = 1;

        let fast_route_len = tsp.get_inverted_route_len(&route, route_len, i, j);

        neighbourhood::invert(&mut route[i..=j]);

        let slow_route_len = tsp.get_route_len(&route).expect("has to be some");

        neighbourhood::invert(&mut route[i..=j]);

        assert_eq!(fast_route_len, slow_route_len);
    }
}
