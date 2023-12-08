use std::{ops::Range, vec};

#[derive(Debug)]
struct InputRange {
    range_start: usize,
    range_end: usize
}

#[derive(Debug)]
struct AlmanacRange {
    destination_range_start: usize,
    source_range_start: usize,
    range_length: usize
}


fn main() {
    let file = std::fs::read_to_string("./input").unwrap();

    let mut input: Vec<InputRange> = vec![];
    let mut almanac: Vec<AlmanacRange> = vec![];

    let mut lines = file.lines();

    while let Some(mut current_line) = lines.next() {
        if current_line.contains("seeds:") {
            let line_split: Vec<&str> = current_line.split("seeds: ").collect();
            let line_nums: Vec<usize> = line_split.get(1)
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect();

            let mut i = 0;
            while i < line_nums.len() {
                input.push(InputRange { range_start: *line_nums.get(i).unwrap(), range_end:  *line_nums.get(i).unwrap() + *line_nums.get(i+1).unwrap() });
                i = i + 2;
            }
        } else {
           while !current_line.is_empty() && !current_line.chars().last().unwrap().is_numeric() {
               current_line = lines.next().unwrap();
           }
           while !current_line.is_empty() && current_line.chars().last().unwrap().is_numeric() {
               let line_nums: Vec<usize> = current_line
                   .split_whitespace()
                   .map(|x| x.parse::<usize>().unwrap())
                   .collect();
               almanac.push(AlmanacRange { 
                   destination_range_start: *line_nums.get(0).unwrap(),
                   source_range_start: *line_nums.get(1).unwrap(),
                   range_length: *line_nums.get(2).unwrap()
               });
               current_line = lines.next().unwrap_or("");
           }
           input = map_input(&input, &almanac);
           almanac = vec![];
        }
    }

    println!("{:?}", input);
}

// fn map_input(input: &Vec<usize>, almanac_ranges: &Vec<AlmanacRange>) -> Vec<usize> {
//     let mut output: Vec<usize> = vec![];
//
//     for input_entry in input {
//         let mut output_entry: usize = *input_entry;
//         for almanac_range in almanac_ranges.into_iter() {
//             let source_range_start = almanac_range.source_range_start;
//             let destination_range_start = almanac_range.destination_range_start;
//             let range_length = almanac_range.range_length; 
//             if (source_range_start..source_range_start + range_length).contains(&input_entry) {
//                 let difference = input_entry - source_range_start;
//                 output_entry = destination_range_start + difference;
//                 break;
//             }
//         }
//         output.push(output_entry);
//     }
//     return output;
// }
fn map_input(input: &Vec<InputRange>, almanac_ranges: &Vec<AlmanacRange>) -> Vec<InputRange> {
    let mut output: Vec<InputRange> = vec![];
    
    for r in input {
        // If no match how do we include that? 
        let mut r_output = InputRange { range_start: r.range_start, range_end: r.range_end };
        for almanac_range in almanac_ranges {
            let source_range_start = almanac_range.source_range_start;
            let destination_range_start = almanac_range.destination_range_start;
            let range_length = almanac_range.range_length; 

            if (r.range_start > source_range_start + range_length) || (source_range_start > r.range_end) {
                continue;
            } else {
                let output_range_start = std::cmp::max(r.range_start, source_range_start);
                let output_range_end = std::cmp::min(r.range_end, source_range_start + range_length); 

                println!("r{:?}", r);
                println!("range_start {}", output_range_start);
                println!("range_end {}", output_range_end);
                let difference = output_range_start - source_range_start;
                let source_start = destination_range_start + difference;
                let source_end = source_start + (output_range_end - output_range_start);

                r_output = InputRange { range_start: source_start, range_end: source_end };
                break;
            }
        }

        output.push(r_output);
    }

    return output;
}
