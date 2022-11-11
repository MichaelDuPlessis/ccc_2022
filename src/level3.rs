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

#[derive(Debug)]
pub struct Ghost {
    pos: Coords,
    movements: String,
}

#[derive(Debug)]
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
        let mut dead = false;

        'outer: for (i, m) in self.movements.chars().enumerate() {
            let new_pos: Coords = match m {
                'U' => self.pacman + Coords(0, -1),
                'D' => self.pacman + Coords(0, 1),
                'L' => self.pacman + Coords(-1, 0),
                'R' => self.pacman + Coords(1, 0),
                _ => Coords(0, 0)
            };

            self.board[self.pacman.1 as usize][self.pacman.0 as usize] = ' ';
            self.pacman = new_pos;

            for g in self.ghosts.iter_mut() {
                let new_pos: Coords = match g.movements.as_bytes()[i] as char {
                    'U' => g.pos + Coords(0, -1),
                    'D' => g.pos + Coords(0, 1),
                    'L' => g.pos + Coords(-1, 0),
                    'R' => g.pos + Coords(1, 0),
                    _ => Coords(0, 0)
                };
    
                if self.board[new_pos.1 as usize][new_pos.0 as usize] == 'P' {
                    dead = true;
                    break 'outer;
                }
    
                // self.board[g.pos.1 as usize][g.pos.0 as usize] = ' ';
                g.pos = new_pos;
            }

            if self.board[new_pos.1 as usize][new_pos.0 as usize] == 'C' {
                count += 1;
                self.board[new_pos.1 as usize][new_pos.0 as usize] = ' ';
            }
        }

        std::fs::write(format!("./{}/{}.out", LEVEL, self.file), format!("{} {}", count.to_string(), if dead { "YES" } else { "NO" })).unwrap();
    }

    pub fn new(file: &str) -> Self {
        let input = std::fs::read_to_string(format!("./{}/{}.in", LEVEL, file)).unwrap();

        let mut lines = input.lines();
        
        let num: usize = lines.next().unwrap().to_string().parse().unwrap();

        let mut array: Vec<Vec<char>> = Vec::new();
        
        let mut coords: Coords = Coords(0, 0);

        let mut pacman_movements = String::new();
        let mut ghost_movements = String::new();


        let mut count = 0;
        let mut ghosts = Vec::new();
        let mut num_ghosts: usize = 0;

                
        let mut check = false;

        let mut g_coords = Coords(0, 0);

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
                pacman_movements = line.trim().to_string();
            }
            else {
                if !check {
                    num_ghosts = line.to_string().parse().unwrap();
                    check = true;
                }
                else {
                    let order = count % 3;

                    match order {
                        0 => {
                            let mut indices = line.split_whitespace();
        
                            g_coords.1 = indices.next().unwrap().to_string().parse::<isize>().unwrap() - 1isize;
                            g_coords.0 = indices.next().unwrap().to_string().parse::<isize>().unwrap() - 1isize;
                        },
                        1 => {},
                        2 => {
                            ghost_movements = line.trim().to_string();

                            let ghost = Ghost {
                                pos: g_coords,
                                movements: ghost_movements.clone(),
                            };

                            ghosts.push(ghost);
                        },
                        _ => {}
                    }
                    
                    
        
                    

                    count += 1;
                } 
            }
        } 

        Self {
            size: num,
            file: file.to_string(),
            board: array,
            movements: pacman_movements,
            pacman: coords,
            ghosts,
        }
    }
}
