use std::collections::VecDeque;

fn main() {
    let mut deq = VecDeque::from([3, 2, 7, 4, 6, 5, 1, 8, 9]);
    for _ in 0..100 {
        lib::do_move_v1(&mut deq);
    }
    while deq.front().unwrap() != &1 {
        deq.rotate_left(1);
    }
    for v in deq.into_iter() {
        if v != 1 {
            print!("{}", v);
        }
    }
    println!();
}
