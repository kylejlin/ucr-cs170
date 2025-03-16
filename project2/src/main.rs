use cs170_project2::*;

fn main() {
    let s = include_str!("./data/CS170_Small_Data__40.txt");
    let dataset = io::parse_dataset(s).unwrap();
    println!("{:#?}", dataset);
}
