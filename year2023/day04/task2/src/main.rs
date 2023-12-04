fn main() {
    let data = lib::read_data();

    let mut cards = vec![1u32; data.len()];

    for (idx, card) in data.iter().enumerate() {
        let matches = card.matches();
        let card_count = cards[idx];
        for other in &mut cards[(idx + 1)..(idx + matches + 1)] {
            *other += card_count;
        }
    }

    println!("{}", cards.iter().sum::<u32>());
}
