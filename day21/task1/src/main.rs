fn simulate<D: Iterator<Item = u16>>(mut positions: Vec<u8>, dice: &mut D) -> (usize, u16) {
    let mut dice = dice.map(|v| v as u16).enumerate().peekable();
    let mut scores = vec![0u16; positions.len()];
    'ext: loop {
        for (pos, score) in positions.iter_mut().zip(scores.iter_mut()) {
            let dice_val = dice.next().unwrap().1 + dice.next().unwrap().1 + dice.next().unwrap().1;
            eprintln!("Dice sum: {}", dice_val);
            *pos = (((*pos as u16 - 1) + dice_val) % 10 + 1) as u8;
            *score += (*pos) as u16;
            if *score >= 1000 {
                break 'ext;
            }
        }
    }
    (
        dice.peek().unwrap().0,
        scores.iter().cloned().find(|&s| s < 1000).unwrap(),
    )
}

fn main() {
    let positions = vec![8, 5];
    // let positions = vec![4, 8];
    let mut dice = 1..;
    let (a, b) = simulate(positions, &mut dice);
    println!("{}", a * (b as usize));
}
