use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("numbers.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut mine_map: HashMap<i128, u32> = HashMap::new();
    let mut mine_vec: VecDeque<i128> = VecDeque::new();

    for (line_count, line) in reader.lines().enumerate() {
        if let Ok(number) = line {
            let number = number.parse::<i128>().unwrap_or_else(|_| {
                panic!("Failed to parse number on line {}", line_count);
            });
            if line_count < 100 {
                *mine_map.entry(number).or_insert(0) += 1;
                mine_vec.push_back(number);
                continue;
            }
            //check if new number is safe
            let is_safe = is_number_safe(&mine_map, number);
            if !is_safe {
                println!("{} at line {} will make mine crumble", number, line_count);
                return;
            }

            //delete first numbers in mine
            if let Some(first_in_mine) = mine_vec.pop_front() {
                if let Some(count) = mine_map.get_mut(&first_in_mine) {
                    if *count == 1 {
                        mine_map.remove(&first_in_mine);
                    } else {
                        *count -= 1;
                    }
                }
            }

            //add new number to mine
            *mine_map.entry(number).or_insert(0) += 1;
            mine_vec.push_back(number);
        }
    }
}
fn is_number_safe(mine_map: &HashMap<i128, u32>, number: i128) -> bool {
    for key in mine_map.keys() {
        let diff = number.checked_sub(*key);
        if let Some(diff) = diff {
            if mine_map.contains_key(&diff) {
                return true;
            }
        }
    }
    false
}
