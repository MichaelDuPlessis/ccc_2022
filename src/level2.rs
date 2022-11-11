#[derive(Clone, Copy)]
pub struct Coords(usize, usize);

impl std::ops::Sub for Coords {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl std::ops::Add for Coords {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

#[derive(Debug)]
pub struct Game {
    size: usize,
    board: Vec<Vec<char>>,
    movements: String,
    pacman: Coords,
}

pub fn count_coins_over(file: &str) {
    let input = std::fs::read_to_string(format!("./level2/{}.in", file)).unwrap();

    let lines = input.lines();

    let mut array: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let new_row = Vec::new();

        for character in line.chars() {
            new_row.push(character);
        }

        array.push(new_row);
    }
}

impl Game {
    fn move_pac(&mut self) {
        let mut count = 0;

        for m in self.movements.chars() {
            let new_pos: Coords = match m {
                'D' => self.pacman + (0, -1),
                'U' => self.pacman + (0, 1),
                'L' => self.pacman + (-1, 0),
                'R' => self.pacman + (0, 1),
                _ => 0
            };

            if self.board[new_pos.0][new_pos.1] == 'C' {
                count += 1;
            }
        }

        std::fs::write("./out", count.to_string())
    }
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
