use crate::level1::count_coins;

pub fn count_coins_over(file: &str) {
    let input = std::fs::read_to_string(format!("./level2/{}.in", file)).unwrap();

    let lines = input.lines();
}