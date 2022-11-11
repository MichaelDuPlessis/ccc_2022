mod level1;
mod level2;

fn main() {
    let game = Game::new("level2_1");

    println!("{:?}", game);

    for i in 1..6 {
        level2::Game::new(&format!("level2_{}", i)).move_pac();
    }

    level2::Game::new(&format!("level2_{}", "example")).move_pac();
}
