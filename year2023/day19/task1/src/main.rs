fn main() {
    let (wfs, items) = lib::read_data();
    let mut sum: u32 = 0;

    for item in &items {
        if lib::execute(&wfs, item) {
            sum += item.values().sum::<u32>();
        }
    }

    println!("{sum}");
}
