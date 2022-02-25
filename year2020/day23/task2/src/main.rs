#[derive(Debug)]
struct Node {
    prev: usize,
    next: usize,
}

impl Node {
    fn new(i: usize, max: usize) -> Self {
        Self {
            prev: if i == 0 { max } else { i - 1 },
            next: if i == max { 0 } else { i + 1 },
        }
    }
}

fn take_from_list(list: &mut [Node], id: usize) -> usize {
    let res = list[id].next;
    let res_next = list[res].next;
    let res_prev = list[res].prev;
    assert_eq!(res_prev, id);
    list[res_next].prev = res_prev;
    list[res_prev].next = res_next;

    list[res].next = 0;
    list[res].prev = 0;
    res
}

fn insert_to_list(list: &mut [Node], after: usize, what: usize) {
    assert_eq!(list[what].next, 0);
    assert_eq!(list[what].prev, 0);

    let after_next = list[after].next;
    list[after].next = what;
    list[what].prev = after;
    list[what].next = after_next;
    list[after_next].prev = what;
}

fn do_move_v2(list: &mut [Node], cur: &mut usize) {
    let v1 = take_from_list(list, *cur);
    let v2 = take_from_list(list, *cur);
    let v3 = take_from_list(list, *cur);

    let dest = if let Some(n) = (1..*cur).rev().find(|&n| list[n].next != 0) {
        n
    } else {
        assert!(list.len() > 4);
        (*cur..list.len())
            .rev()
            .find(|&n| n != 0 && list[n].next != 0)
            .unwrap()
    };
    assert_ne!(list[dest].next, 0);
    assert_ne!(list[dest].prev, 0);

    insert_to_list(list, dest, v3);
    insert_to_list(list, dest, v2);
    insert_to_list(list, dest, v1);

    *cur = list[*cur].next;
}

fn main() {
    const SIZE: usize = 1_000_000;

    let mut list = (0..=SIZE).map(|i| Node::new(i, SIZE)).collect::<Vec<_>>();

    let seq = [3, 2, 7, 4, 6, 5, 1, 8, 9];
    for (&id1, &id2) in (&seq).iter().zip((&seq).iter().skip(1)) {
        list[id1].next = id2;
        list[id2].prev = id1;
    }
    list[SIZE].next = seq[0];
    list[seq[0]].prev = SIZE;
    let last = seq[seq.len() - 1];
    let nxt = seq.len() + 1;
    list[last].next = nxt;
    list[nxt].prev = last;

    let mut cur = seq[0];
    for _ in 0..10_000_000 {
        do_move_v2(&mut list, &mut cur);
    }

    let v1 = list[1].next;
    let v2 = list[v1].next;
    println!("{}", v1 * v2);
}
