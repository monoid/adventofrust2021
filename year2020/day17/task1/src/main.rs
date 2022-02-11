use std::io;

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let mut scene = lib::Scene3::new(stdin);

    for _ in 0..6 {
        scene = scene.advance();
    }

    println!("{}", scene.len());
}
