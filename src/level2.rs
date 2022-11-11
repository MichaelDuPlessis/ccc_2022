use crate::level1::count_coins;

type Coords = (usize, usize);

struct Game {
    size: usize,
    board: Vec<Vec<char>>,
    movements: String,
    pacman: Coords,
}

pub fn count_coins_over(file: &str) {



}

impl Game {
    pub fn new(file: &str) -> Self {
        let input = std::fs::read_to_string(format!("./level2/{}.in", file)).unwrap();

        let mut lines = input.lines();
        
        let num: usize = lines.next().unwrap().to_string().parse().unwrap();

        let mut array: Vec<Vec<char>> = Vec::new();
        
        let mut coords: Coords = (0, 0);

        let mut _movements = String::new();

        for (index, line) in lines.enumerate() {
            if index < 10 {
                let mut new_row = Vec::new();
    
                for character in line.chars() {
                    new_row.push(character);
                }
    
                array.push(new_row);
            }
            else if index == 10 {
                let mut indeces = line.split_whitespace();

                coords.0 = indeces.next().unwrap().to_string().parse().unwrap();
                coords.1 = indeces.next().unwrap().to_string().parse().unwrap();
            }
            else {
                _movements = line.trim().to_string();
            }

        } 

        Self {
            size: num,
            board: array,
            movements: _movements,
            pacman: coords,
        }
    }
}
