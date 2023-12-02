use lib::{parse_game, situation};

fn main() {
    let sum = std::io::stdin()
        .lines()
        .filter_map(|line| {
            let line = line.unwrap();
            let line = line.trim();
            let game = parse_game(line).unwrap().1;
            let situation = situation();
            if game.is_possible(&situation) {
                Some(game.id)
            } else {
                None
            }
        })
        .sum::<u32>();

    println!("{}", sum);
}
