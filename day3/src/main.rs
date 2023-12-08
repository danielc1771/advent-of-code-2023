fn main() {
    let file = std::fs::read_to_string("./input").unwrap();
    
    let mut parts: Vec<Vec<char>> = vec![];

    file
        .lines()
        .for_each(|line| add_to_parts(line, &mut parts));

    let m = parts.len();
    let n = parts.get(0).unwrap().len();

    let mut i = 0;
    let mut j = 0;
    let mut ans = 0;
     while i < m {
        while j < n {
            let curr = parts.get(i).unwrap().get(j).unwrap();

            if curr.to_string() == "*" {
                ans += check_valid(i, j, j, &parts);
            }
            j += 1;
        }
        j = 0;
        i += 1;
    }
     println!("{}", ans);
}

fn check_valid(row: usize, start_col: usize, end_col: usize, parts: &Vec<Vec<char>>) -> usize {
    let i = row;

    let mut numbers: Vec<(usize, usize, usize)> = vec![];
    for j in start_col..end_col + 1 {
        if i > 0 {
            if let Some(above_row) = parts.get(i - 1) {
                if j > 0 {
                    if let Some(val) = above_row.get(j - 1) {
                        if val.is_numeric() {
                            add_number(i - 1, j-1, &mut numbers, &parts);
                        }
                    }
                }
                if let Some(val) = above_row.get(j) {
                    if val.is_numeric() {
                        add_number(i - 1, j, &mut numbers, &parts);
                    }
                }
                if let Some(val) = above_row.get(j + 1) {
                    if val.is_numeric() {
                        add_number(i - 1, j+1, &mut numbers, &parts);
                    }
                }
            }
        }
        if let Some(below_row) = parts.get(i + 1) {
            if j > 0 {
                if let Some(val) = below_row.get(j - 1) {
                    if val.is_numeric() {
                        add_number(i + 1, j-1, &mut numbers, &parts);
                    }
                }
            }
            if let Some(val) = below_row.get(j) {
                if val.is_numeric() {
                    add_number(i + 1, j, &mut numbers, &parts);
                }
            }
            if let Some(val) = below_row.get(j + 1) {
                if val.is_numeric() {
                    add_number(i + 1, j+1, &mut numbers, &parts);
                }
            }
        }
        let current_row = parts.get(row).unwrap();
        if j > 0 {
            if let Some(val) = current_row.get(j - 1) {
                if val.is_numeric() {
                    add_number(i, j-1, &mut numbers, &parts);
                }
            }
        }
        if let Some(val) = current_row.get(j + 1) {
            if val.is_numeric() {
                add_number(i, j+1, &mut numbers, &parts);
            }
        }
    }

    if numbers.len() == 2 {
        let (i_1, j_start_1, j_end_1) = numbers.get(0).unwrap();
        let (i_2, j_start_2, j_end_2) = numbers.get(1).unwrap();

        let mut num_1_string = String::new();
        let mut num_2_string = String::new();
        
        let row1 = parts.get(*i_1).unwrap();
        let row2 = parts.get(*i_2).unwrap();

        for j in *j_start_1..*j_end_1 + 1 {
            num_1_string.push(*row1.get(j).unwrap());
        }
        for j in *j_start_2..*j_end_2 + 1 {
            num_2_string.push(*row2.get(j).unwrap());
        }
        let num1:usize = num_1_string.parse().unwrap();    
        let num2:usize = num_2_string.parse().unwrap();    
        return  num1 * num2;
    }
    return 0;
}

fn add_number(row: usize, col: usize, nums: &mut Vec<(usize, usize, usize)>, parts: &Vec<Vec<char>>) {
    let mut start_idx = col;
    let mut end_idx = col;

    let mut i = col;

    while i > 0 {
        i -= 1;
        if parts.get(row).unwrap().get(i).unwrap().is_numeric() {
            start_idx = i;
        } else {
            break;
        }
    }
    i = col;
    while i < parts.get(0).unwrap().len() {
        if parts.get(row).unwrap().get(i).unwrap().is_numeric() {
            end_idx = i;
        } else {
            break;
        }
        i += 1;
    }
    
    for i in 0..nums.len() {
        let (i, start, end) = nums.get(i).unwrap(); 
        if *i == row && *start == start_idx && *end == end_idx {
            return;
        }
    }
    nums.push((row, start_idx, end_idx));
}

fn add_to_parts(line: &str, parts: &mut Vec<Vec<char>>) {
    parts.push(line.chars().collect());
}
