use advent_of_code_2022_08::count_visible;
use std::env;
use std::fs;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read {file_path}");

    println!("There are {} trees visible.", count_visible(&contents));
}
