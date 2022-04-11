use rand::prelude::*;

#[derive(Debug)]
pub enum TspRouteError {
    TooShort,
    NotPermutation,
}

#[derive(Debug)]
pub struct Tsp {
    edges: Vec<Vec<u32>>,
    dimension: usize,
}

impl Tsp {
    pub fn new(edges: Vec<Vec<u32>>, dimension: usize) -> Tsp {
        Tsp { edges, dimension }
    }

    pub fn get_route_len(&self, route: &[usize]) -> Result<u32, TspRouteError> {
        self.check_route_valid(route)?;

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

    fn check_route_valid(&self, route: &[usize]) -> Result<(), TspRouteError> {
        if route.len() != self.dimension {
            return Err(TspRouteError::TooShort);
        }

        let mut route_clone = route.to_vec();

        route_clone.sort_unstable();

        if route_clone != (0..self.dimension).collect::<Vec<_>>() {
            return Err(TspRouteError::NotPermutation);
        }

        Ok(())
    }

    pub fn get_edges(self) -> Vec<Vec<u32>> {
        self.edges
    }

    pub fn k_random_route(&self, k: usize) -> Vec<usize> {
        let mut route = (0..self.dimension).collect::<Vec<_>>();
        let mut best_route = None;
        let mut best_route_len = None;
        let mut rng = rand_pcg::Pcg64Mcg::new(thread_rng().gen());

        for _ in 0..k {
            route.shuffle(&mut rng);
            let route_len = self.get_route_len(&route).expect("has to be valid route");

            if (best_route.is_none() && best_route_len.is_none())
                || route_len < best_route_len.unwrap()
            {
                best_route = Some(route.clone());
                best_route_len = Some(route_len);
            }
        }

        best_route.expect("there has to be some route")
    }

    pub fn nearest_neighbour_route(&self) -> Vec<usize> {
        let first_vertex = thread_rng().gen_range(0..self.dimension);

        self.nearest_neighbour_inner(first_vertex)
    }

    pub fn nearest_neighbour_optimized_route(&self) -> Vec<usize> {
        let mut best_route = None;
        let mut best_route_len = None;

        for i in 0..self.dimension {
            let route = self.nearest_neighbour_inner(i);
            let route_len = self.get_route_len(&route).expect("has to be valid route");

            if (best_route.is_none() && best_route_len.is_none())
                || route_len < best_route_len.unwrap()
            {
                best_route = Some(route);
                best_route_len = Some(route_len);
            }
        }

        best_route.expect("has to be valid route")
    }

    pub fn two_opt(&self) -> Vec<usize> {
        let initial_route = self.nearest_neighbour_optimized_route();
        self.best_neighbourhood(initial_route, Tsp::invert_route)
    }

    fn best_neighbourhood<F>(&self, route: Vec<usize>, neighbourhood: F) -> Vec<usize>
    where
        F: Fn(&mut [usize]),
    {
        let mut best_route = route;
        let mut best_route_len = self
            .get_route_len(&best_route)
            .expect("has to be valid route");

        let mut indexes = None;
        let mut curr_best_route_len = None;

        loop {
            for i in 0..self.dimension {
                for j in i..self.dimension {
                    neighbourhood(&mut best_route[i..=j]);
                    let route_len = self.get_route_len(&best_route).expect("has to be some");

                    if (curr_best_route_len.is_none() && route_len < best_route_len)
                        || (curr_best_route_len.is_some()
                            && route_len < curr_best_route_len.unwrap())
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

            Tsp::invert_route(&mut best_route[i..=j]);
            best_route_len = curr_best_route_len.unwrap();

            indexes = None;
            curr_best_route_len = None;
        }

        best_route
    }

    fn nearest_neighbour_inner(&self, starting_vertex: usize) -> Vec<usize> {
        assert!(
            starting_vertex < self.dimension,
            "Vertex bigger than dimension"
        );

        let mut route = vec![0; self.dimension];
        let mut visited = vec![false; self.dimension];

        route[0] = starting_vertex;
        visited[starting_vertex] = true;

        let mut curr_vertex = starting_vertex;

        for i in 1..self.dimension {
            let mut min_len = u32::MAX;
            let mut next_vertex = 0;

            for i in 0..self.dimension {
                if visited[i] {
                    continue;
                }

                let curr_len = self.edges[curr_vertex][i];

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

    fn invert_route(route: &mut [usize]) {
        let route_len = route.len();
        let half_route_len = route_len / 2;

        for i in 0..=half_route_len {
            route.swap(i, route_len - i - 1);
        }
    }

    // fn swap_route(route: &mut [usize]) {
    //     route.swap(0, route.len() - 1);
    // }
}
