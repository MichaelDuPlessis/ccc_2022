use crate::level1::count_coins;

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