const TOP: usize = 3;

fn main() {
    let lines = lib::lines().map(Result::unwrap);
    let mut elves: Vec<u32> = lib::elves(lines).into_iter().map(|elf| elf.calories()).collect();
    let top = if elves.len() > TOP {
        let len = elves.len();
        elves.select_nth_unstable(len - TOP - 1).2
    } else {
        &elves[..]
    };
    println!("{}", top.iter().sum::<u32>());
}
