use lib::Command;
use std::ops::RangeInclusive;

fn apply_commands(
    commands: &[Command],
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
    z: RangeInclusive<i32>,
) -> usize {
    let mut state =
        vec![vec![vec![false; z.clone().count()]; y.clone().count()]; x.clone().count()];

    for c in commands {
        // I do not bother with any optimization, writing as short
        // as possible instead.
        for cx in c.cuboid.x.clone() {
            if x.contains(&cx) {
                let ix = cx - *x.start();
                for cy in c.cuboid.y.clone() {
                    if y.contains(&cy) {
                        let iy = cy - *y.start();
                        for cz in c.cuboid.z.clone() {
                            if z.contains(&cy) {
                                let iz = cz - *z.start();
                                state[ix as usize][iy as usize][iz as usize] = c.state;
                            }
                        }
                    }
                }
            }
        }
    }

    state
        .into_iter()
        .map(|yline| {
            yline
                .into_iter()
                .map(|zline| zline.into_iter().filter(|s| *s).count())
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    let commands = lib::prase_input();
    let res = apply_commands(&commands, -50..=50, -50..=50, -50..=50);
    println!("{}", res);
}
