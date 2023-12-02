use lib::parse_game;

fn main() {
    let sum = std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let line = line.trim();
            let game = parse_game(line).unwrap().1;
            let min_situation = game.min_situation();
            min_situation.values().cloned().product::<u32>()
        })
        .sum::<u32>();

    println!("{}", sum);
}
