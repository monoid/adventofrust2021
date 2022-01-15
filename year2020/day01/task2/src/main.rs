const YEAR: i32 = 2020;

fn main() {
    let data = lib::read_input();

    // O(n^3) method, but it is good enough.
    for i in 0..data.len() - 2 {
        for j in (i + 1) .. data.len() - 1 {
            for k in (j + 1) .. data.len() {
                if data[i] + data[j] + data[k] == YEAR {
                    println!("{}", data[i] * data[j] * data[k]);
                    return;
                }
            }
        }
    }
}
