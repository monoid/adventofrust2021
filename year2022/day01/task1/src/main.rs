fn main() {
    let lines = lib::lines().map(Result::unwrap);
    let elves = lib::elves(lines).into_iter().map(|items| items.calories());
    println!("{}", elves.max().unwrap());
}
