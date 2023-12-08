fn main() {
    let file = std::fs::read_to_string("./input").unwrap();
    let mut ans = 0;
    file.lines().for_each(|line| ans += validate_game(line));
    
    println!("{}", ans);
}

fn validate_game(line: &str) -> usize {
    let games_vec: Vec<&str> = line
        .split(": ")
        .last()
        .unwrap()
        .split(";")
        .map(|game| game.trim())
        .collect();

    let mut max_green = 1; 
    let mut max_blue = 1; 
    let mut max_red = 1;
    for game in games_vec {
        let colors: Vec<&str> = game.split(",").map(|color| color.trim()).collect();

        for color_str in colors {
            let color_str_vec: Vec<&str> = color_str.split_whitespace().collect();
            let color_amount: usize = color_str_vec.get(0).unwrap().parse().unwrap();
            let color = color_str_vec.get(1).unwrap().to_string();

            if color == "green" {
                max_green = std::cmp::max(max_green, color_amount);
            }
            if color == "red" {
                max_red = std::cmp::max(max_red, color_amount);
            }
            if color == "blue" {
                max_blue = std::cmp::max(max_blue, color_amount);
            }
        }
    }
    return max_red * max_green * max_blue;
}
