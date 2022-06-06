#[derive(Debug)]
pub enum TspRouteError {
    TooShort,
    NotPermutation,
}

#[derive(Debug, Clone, Copy)]
pub enum TspType {
    Symmetric,
    Asymmetric,
}

#[derive(Debug, Clone)]
pub struct Tsp {
    edges: Vec<Vec<u32>>,
    dimension: usize,
    tsp_type: TspType,
}

impl Tsp {
    pub fn new(edges: Vec<Vec<u32>>, dimension: usize, tsp_type: TspType) -> Tsp {
        Tsp {
            edges,
            dimension,
            tsp_type,
        }
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
        route_len: u32,
        i: usize,
        j: usize,
    ) -> u32 {
        match self.tsp_type {
            TspType::Symmetric => self.get_inverted_symmetric_route_len(route, route_len, i, j),
            TspType::Asymmetric => self.get_inverted_asymmetric_route_len(route, route_len, i, j),
        }
    }

    pub fn get_inverted_symmetric_route_len(
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

    pub fn get_swap_route_len(
        &self,
        route: &[usize],
        mut route_len: u32,
        i: usize,
        j: usize,
    ) -> u32 {
        let before_i_index = if i == 0 { self.dimension - 1 } else { i - 1 };
        let after_i_index = i + 1;
        let before_j_index = j - 1;
        let after_j_index = if j == self.dimension - 1 { 0 } else { j + 1 };

        route_len -= self.edges[route[before_i_index]][route[i]];
        route_len -= self.edges[route[i]][route[after_i_index]];
        route_len -= self.edges[route[j]][route[after_j_index]];

        if after_i_index != j {
            route_len -= self.edges[route[before_j_index]][route[j]];

            route_len += self.edges[route[before_j_index]][route[i]];
            route_len += self.edges[route[j]][route[after_i_index]];
        } else {
            route_len += self.edges[route[j]][route[i]];
        }

        route_len += self.edges[route[before_i_index]][route[j]];
        route_len += self.edges[route[i]][route[after_j_index]];

        route_len
    }

    // ==

    pub fn get_inverted_asymmetric_route_len(
        &self,
        route: &[usize],
        mut route_len: u32,
        i: usize,
        j: usize,
    ) -> u32 {
        let before_index = if i == 0 { self.dimension - 1 } else { i - 1 };
        let after_index = if j == self.dimension - 1 { 0 } else { j + 1 };

        route_len += self.edges[route[i]][route[after_index]];
        route_len += self.edges[route[before_index]][route[j]];

        route_len -= self.edges[route[before_index]][route[i]];
        route_len -= self.edges[route[j]][route[after_index]];

        route_len -= self.get_part_route_len(&route[i..=j]);
        route_len += self.get_inverted_part_route_len(&route[i..=j]);

        route_len
    }

    fn get_part_route_len(&self, route: &[usize]) -> u32 {
        let mut route_len = 0;

        for i in 0..route.len() - 1 {
            let first_vertex = route[i];
            let second_vertex = route[i + 1];

            route_len += self.edges[first_vertex][second_vertex];
        }

        route_len
    }

    fn get_inverted_part_route_len(&self, route: &[usize]) -> u32 {
        let mut route_len = 0;

        for i in (0..route.len() - 1).rev() {
            let first_vertex = route[i + 1];
            let second_vertex = route[i];

            route_len += self.edges[first_vertex][second_vertex];
        }

        route_len
    }

    // ===

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

    pub fn get_tsp_type(&self) -> TspType {
        self.tsp_type
    }
}

#[cfg(test)]
mod tests {
    use crate::neighbourhood::{invert, swap};
    use crate::{Tsp, TspParser};

    #[test]
    fn different_route_lens() {
        let tsp = TspParser::from_file("test_files/bier127.tsp").expect("test file doesnt exist");

        let mut route = (0..tsp.dimension).collect::<Vec<_>>();

        let route_len = tsp.get_route_len(&route).expect("has to be some");

        let (l, r) = (1, 2);

        let other_route_len = tsp.get_inverted_route_len(&route, route_len, l, r);

        invert(&mut route[l..=r]);

        let inverted_route_len = tsp.get_route_len(&route).expect("has to be some");

        assert_eq!(inverted_route_len, other_route_len);
    }

    fn check_swap(tsp: &Tsp, l: usize, r: usize) {
        let mut route = (0..tsp.dimension).collect::<Vec<_>>();

        let route_len = tsp.get_route_len(&route).expect("has to be some");

        let other_route_len = tsp.get_swap_route_len(&route, route_len, l, r);

        swap(&mut route[l..=r]);

        let swapped_route_len = tsp.get_route_len(&route).expect("has to be some");

        assert_eq!(swapped_route_len, other_route_len);
    }

    #[test]
    fn different_route_lens_swap() {
        let tsp = TspParser::from_file("test_files/ft70.atsp").expect("test file doesnt exist");

        check_swap(&tsp, 10, 40);
        check_swap(&tsp, 5, 7);
        check_swap(&tsp, 5, 6);
        check_swap(&tsp, 1, 2);
        check_swap(&tsp, 50, 60);
        check_swap(&tsp, 45, 55);
    }
}
