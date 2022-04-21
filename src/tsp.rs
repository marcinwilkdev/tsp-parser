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

    pub fn get_inverted_route_len(
        &self,
        route: &[usize],
        mut route_len: u32,
        i: usize,
        j: usize,
    ) -> u32 {
        if i == 0 && j == self.dimension - 1 {
            return route_len;
        }

        let before_index = if i == 0 { self.dimension - 1 } else { i - 1 };
        let after_index = if j == self.dimension - 1 { 0 } else { j + 1 };

        route_len += self.edges[route[i]][route[after_index]];
        route_len += self.edges[route[before_index]][route[j]];

        route_len -= self.edges[route[before_index]][route[i]];
        route_len -= self.edges[route[j]][route[after_index]];

        route_len
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

    pub fn get_edges(&self) -> &[Vec<u32>] {
        &self.edges
    }

    pub fn get_dimension(&self) -> usize {
        self.dimension
    }
}
