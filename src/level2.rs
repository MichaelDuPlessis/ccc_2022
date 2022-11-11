use crate::level1::count_coins;

type Coords = (usize, usize);

struct Game {
    size: usize,
    board: Vec<Vec<char>>,
    movements: String,
    pacman: Coords,
}

pub fn count_coins_over(file: &str) {
    let input = std::fs::read_to_string(format!("./level2/{}.in", file)).unwrap();

    let lines = input.lines();
}






