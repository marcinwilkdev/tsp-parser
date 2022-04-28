pub fn invert(route: &mut [usize]) {
    let route_len = route.len();
    let half_route_len = route_len / 2;

    for i in 0..half_route_len {
        route.swap(i, route_len - i - 1);
    }
}

pub fn swap(route: &mut [usize]) {
    route.swap(0, route.len() - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invert_even_works() {
        let mut route = [1, 2, 3, 4, 5, 6];

        invert(&mut route[1..=4]);

        assert_eq!([1, 5, 4, 3, 2, 6], route);
    }

    #[test]
    fn invert_odd_works() {
        let mut route = [1, 2, 3, 4, 5];

        invert(&mut route[1..=3]);

        assert_eq!([1, 4, 3, 2, 5], route);
    }

    #[test]
    fn swap_even_works() {
        let mut route = [1, 2, 3, 4, 5, 6];

        swap(&mut route[1..=4]);

        assert_eq!([1, 5, 3, 4, 2, 6], route);
    }

    #[test]
    fn swap_odd_works() {
        let mut route = [1, 2, 3, 4, 5];

        swap(&mut route[1..=3]);

        assert_eq!([1, 4, 3, 2, 5], route);
    }
}
