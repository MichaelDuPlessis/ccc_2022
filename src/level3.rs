const LEVEL: &str = "level3";

#[derive(Clone, Copy, Debug)]
pub struct Coords(isize, isize);

impl std::ops::Sub for Coords {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl std::ops::Add for Coords {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

pub struct Ghost {
    pos: Coords,
    movements: String,
}

pub struct Game {
    file: String,
    size: usize,
    board: Vec<Vec<char>>,
    movements: String,
    pacman: Coords,
    ghosts: Vec<Ghost>,
}

impl Game {
    pub fn move_pac(&mut self) {
        let mut count = 0;

        for m in self.movements.chars() {
            let new_pos: Coords = match m {
                'U' => self.pacman + Coords(0, -1),
                'D' => self.pacman + Coords(0, 1),
                'L' => self.pacman + Coords(-1, 0),
                'R' => self.pacman + Coords(1, 0),
                _ => Coords(0, 0)
            };

            println!("{:?}", new_pos);

            if self.board[new_pos.1 as usize][new_pos.0 as usize] == 'C' {
                count += 1;
                self.board[new_pos.1 as usize][new_pos.0 as usize] = ' ';
            }

            self.pacman = new_pos;
        }

        std::fs::write(format!("./{}/{}.out", LEVEL, self.file), count.to_string()).unwrap();
    }

    pub fn new(file: &str) -> Self {
        let input = std::fs::read_to_string(format!("./{}/{}.in", LEVEL, file)).unwrap();

        let mut lines = input.lines();
        
        let num: usize = lines.next().unwrap().to_string().parse().unwrap();

        let mut array: Vec<Vec<char>> = Vec::new();
        
        let mut coords: Coords = Coords(0, 0);

        let mut _movements = String::new();

        for (index, line) in lines.enumerate() {
            if index < num {
                let mut new_row = Vec::new();
    
                for character in line.chars() {
                    new_row.push(character);
                }
    
                array.push(new_row);
            }
            else if index == num {
                let mut indeces = line.split_whitespace();

                coords.1 = indeces.next().unwrap().to_string().parse::<isize>().unwrap() - 1isize;
                coords.0 = indeces.next().unwrap().to_string().parse::<isize>().unwrap() - 1isize;
            }
            else if index <= num + 2{
                _movements = line.trim().to_string();
            }
            else {
                break;
            }
        } 

        let mut num_ghosts: usize = lines.next().unwrap().to_string().parse().unwrap();

        let mut ghosts = Vec::new();

        for line in lines {
            let mut g_coords = Coords(0, 0);

            let mut indices = line.split_whitespace();

            g_coords.1 = indeces.next().unwrap().to_string().parse::<isize>().unwrap() - 1isize;
            g_coords.0 = indeces.next().unwrap().to_string().parse::<isize>().unwrap() - 1isize;

            let _movements = lines.next().unwrap().trim().to_string();

            let ghost = Ghost {
                pos: g_coords,
                movements: _movements,
            };

            ghosts.push(ghost);
        }

        Self {
            size: num,
            file: file.to_string(),
            board: array,
            movements: _movements,
            pacman: coords,
            ghosts,
        }
    }
}
