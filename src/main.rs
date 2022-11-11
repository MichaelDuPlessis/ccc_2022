mod level1;
mod level2;
pub mod level3;

fn main() {
<<<<<<< HEAD

=======
>>>>>>> a4dde63fb30e68ffaa605d424a8bda938287d92f
    for i in 1..6 {
        level3::Game::new(&format!("level3_{}", i)).move_pac();
    }

    level2::Game::new(&format!("level3_{}", "example")).move_pac();
}
