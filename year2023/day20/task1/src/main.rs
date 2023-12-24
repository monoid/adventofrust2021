fn main() {
    let mut modules = lib::read_data();
    let mut sum_a = 0;
    let mut sum_b = 0;
    for _ in 0..1000 {
        let (a, b) = lib::execute(&mut modules);
        sum_a += a;
        sum_b += b;
    }
    println!("{}", sum_a * sum_b);
}
