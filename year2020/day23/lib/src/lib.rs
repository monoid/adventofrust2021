use std::collections::VecDeque;

pub fn do_move_v1(deq: &mut VecDeque<u32>) {
    let cur = deq.pop_front().unwrap();
    let v1 = deq.pop_front().unwrap();
    let v2 = deq.pop_front().unwrap();
    let v3 = deq.pop_front().unwrap();

    // find insertion
    if let Some((idx, _)) = deq
        .iter()
        .cloned()
        .enumerate()
        .filter(|&(_, v)| v < cur)
        .max_by_key(|(_, v)| *v)
    {
        deq.insert(idx + 1, v3);
        deq.insert(idx + 1, v2);
        deq.insert(idx + 1, v1);
    } else {
        let (idx, _) = deq
            .iter()
            .cloned()
            .enumerate()
            .max_by_key(|(_, v)| *v)
            .unwrap();
        deq.insert(idx + 1, v3);
        deq.insert(idx + 1, v2);
        deq.insert(idx + 1, v1);
    }
    deq.push_back(cur);
}
