pub fn count_coins(file: &str) {
    let input = std::fs::read_to_string(format!("./level1/{}.in", file)).unwrap();

    let coins = input.chars().filter(|c| *c == 'C').count();

    std::fs::write(format!("./level1/{}.out", file), coins.to_string()).unwrap();
}