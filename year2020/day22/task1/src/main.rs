use std::io;

fn main() {
    let (mut d1, mut d2) = lib::read_data(io::stdin().lock());

    while !(d1.is_empty() || d2.is_empty()) {
        let v1 = d1.pop_front().unwrap();
        let v2 = d2.pop_front().unwrap();

        if v1 > v2 {
            d1.push_back(v1);
            d1.push_back(v2);
        } else {
            d2.push_back(v2);
            d2.push_back(v1);
        }
    }

    println!("{}", lib::score(if d1.is_empty() { &d2 } else { &d1 }));
}
