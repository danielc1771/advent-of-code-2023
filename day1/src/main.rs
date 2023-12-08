use std::fs;

fn main() {
    let file = fs::read_to_string("./input").unwrap();
    let contents = file.split("\n");

    let ans: usize = contents.map(|x| get_value(x)).sum();

    println!("{}", ans);
}

fn get_value(line: &str) -> usize {
    let chars: Vec<char> = line.chars().collect();

    let only_numbers: Vec<char> = chars
        .into_iter()
        .filter(|x| x.is_numeric())
        .collect();

    if only_numbers.len() == 0 {
        return 0;
    }

    let first = only_numbers.first().unwrap();
    let last = only_numbers.last().unwrap();
    let mut combined = String::new();
    combined.push(*first);
    combined.push(*last);

    return combined.parse::<usize>().unwrap();
}
