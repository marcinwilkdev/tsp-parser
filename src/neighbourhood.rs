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
