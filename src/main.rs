use tsp_parser::Tsp;

fn main() {
    let tsp = Tsp::from_file("test_files/a280.tsp");

    if let Ok(tsp) = tsp {
        let edges = tsp.get_edges();

        println!("{:?}", edges);
    }
}
