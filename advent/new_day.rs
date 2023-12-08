fn main() {
    let file = std::fs::read_to_string("./input").unwrap();

    file
        .lines()
        .for_each(|line| println!("{}", line));
}
