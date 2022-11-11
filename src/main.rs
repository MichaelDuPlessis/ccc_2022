mod level1;
mod level2;

fn main() {
    let game = Game::new("level2_1");

    println!("{:?}", game);

    for i in 1..6 {
        level1::count_coins(&format!("level1_{}", i));
    }
}
