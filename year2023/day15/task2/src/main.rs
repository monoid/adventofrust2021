fn main() {
    let data = lib::read_data_v2();
    let mut boxes: Vec<Vec<lib::Lens>> = vec![vec![]; 256];

    for cmd in data {
        match cmd {
            lib::Command::Add(label, power) => {
                let h = lib::hash(label.as_bytes());
                let bx = &mut boxes[h as usize];
                if let Some(exitsing) = bx.iter_mut().find(|b| b.label == label) {
                    exitsing.power = power;
                } else {
                    bx.push(lib::Lens { label, power })
                }
            }
            lib::Command::Remove(label) => {
                let h = lib::hash(label.as_bytes());
                let bx = &mut boxes[h as usize];
                bx.retain(|l| l.label != label);
            }
        }
    }

    let value: u32 = boxes
        .iter()
        .enumerate()
        .map(|(idx, bx)| (idx as u32 + 1) * box_power(bx))
        .sum();
    println!("{value}");
}

fn box_power(bx: &[lib::Lens]) -> u32 {
    bx.iter()
        .enumerate()
        .map(|(idx, l)| (idx as u32 + 1) * l.power)
        .sum()
}
