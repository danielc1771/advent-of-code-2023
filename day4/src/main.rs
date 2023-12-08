use std::collections::HashMap;

fn main() {
    let file = std::fs::read_to_string("./input").unwrap();
    let mut games: HashMap<usize, &str> = HashMap::new();
    let mut games_to_process: Vec<&str> = vec![];
    let mut games_depth: HashMap<usize, usize> = HashMap::new();

    file
        .lines()
        .enumerate()
        .for_each(|(idx, line)| {
            games.insert(idx, line);
            games_to_process.push(line);
        });

    games_to_process.iter()
        .enumerate()
        .for_each(|(idx, _)| {
            get_depth(idx + 1, &mut games_depth, &games);
        });

    // while !games_to_process.is_empty() {
    //     ans += 1;
    //     let games_to_add = process(games_to_process.remove(0));
    //
    //     for game in games_to_add {
    //         if let Some(potenital_game) = games.get(&(game - 1)) {
    //             games_to_process.push(potenital_game);
    //         }
    //     }
    // }

    let sum: usize = games_depth.values().sum();
    println!("{:?}", sum);
}


fn get_depth(game_num: usize, games_depth: &mut HashMap<usize, usize>, games: &HashMap<usize, &str>) -> usize {

    if games_depth.contains_key(&game_num) {
        return *games_depth.get(&game_num).unwrap();
    }

    let line = games.get(&(game_num - 1)).unwrap();
    let colon_split: Vec<&str> = line.split(":").collect();
    let bar_split: Vec<&str> = colon_split.get(1).unwrap().split("|").collect();

    let winning_numbers: Vec<&str> = bar_split.get(0).unwrap().trim().split_whitespace().collect();
    let actual_numbers: Vec<&str> = bar_split.get(1).unwrap().trim().split_whitespace().collect();
    
    let mut depth = 1;
    let mut i = 1;
    for num in actual_numbers {
        if winning_numbers.contains(&num) {
            depth += get_depth(game_num + i, games_depth, games);
            i += 1;
        }
    }
    
    games_depth.insert(game_num, depth);

    return depth;
}

fn process(line: &str) -> Vec<usize> {

    let colon_split: Vec<&str> = line.split(":").collect();
    let game_string: Vec<&str> = colon_split.get(0).unwrap().split_whitespace().collect();
    let game_number: usize = game_string.get(1).unwrap().trim().parse().unwrap();

    let bar_split: Vec<&str> = colon_split.get(1).unwrap().split("|").collect();

    let winning_numbers: Vec<&str> = bar_split.get(0).unwrap().trim().split_whitespace().collect();
    let actual_numbers: Vec<&str> = bar_split.get(1).unwrap().trim().split_whitespace().collect();
    
    let mut matches = 0;

    for num in actual_numbers {
        if winning_numbers.contains(&num) {
            matches += 1;
        }
    }

    let mut ans: Vec<usize> = vec![];

    for i in 1..matches + 1 {
        ans.push(game_number + i);
    }

    return ans;
}

