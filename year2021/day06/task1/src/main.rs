use std::{
    collections::VecDeque,
    io::{self, BufRead as _},
};

fn do_day(v: &mut VecDeque<u32>) {
    let new = v.pop_front().unwrap();
    v.push_back(new); // newborn
    v[6] += new; // Those who gave birth
}

fn distr_to_counters(v: impl Iterator<Item = u8>) -> VecDeque<u32> {
    let mut counts = vec![0u32; 9];
    for i in v {
        counts[i as usize] += 1;
    }
    VecDeque::from(counts)
}

fn main() {
    let mut num_str = String::new();
    io::stdin().lock().read_line(&mut &mut num_str).unwrap();
    let ages = num_str.trim().split(',').map(|s| s.parse::<u8>().unwrap());
    let mut counters_deq = distr_to_counters(ages);
    for _ in 0..80 {
        do_day(&mut counters_deq);
    }

    println!("{}", counters_deq.into_iter().sum::<u32>());
}
