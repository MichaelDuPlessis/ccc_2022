mod level1;

fn main() {
    for i in 1..6 {
        level1::count_coins(&format!("level1_{}", i));
    }
}
