use std::collections::BinaryHeap;

use crate::Map;

type Pos = (usize, usize);

#[derive(Debug, Clone)]
struct State {
    path: im::HashSet<Pos>,
    last: Pos,
}

impl State {
    fn new(start: Pos) -> Self {
        Self {
            path: im::HashSet::unit(start),
            last: start,
        }
    }

    fn heur(&self, map: &Map) -> usize {
        let len = self.path.len();
        let last = self.last;
        let dx = map.start.0.abs_diff(last.0);
        let dy = map.start.1.abs_diff(last.1);

        len + dx + dy
    }

    fn try_extend(&self, pos: Pos) -> Option<Self> {
        if self.last == pos || self.path.contains(&pos) {
            None
        } else {
            Some(Self {
                path: self.path.update(pos),
                last: pos,
            })
        }
    }
}

#[derive(Debug)]
struct Pair<K, V> {
    key: K,
    value: V,
}

impl<K, V> Pair<K, V> {
    fn new(key: K, value: V) -> Self {
        Self { key, value }
    }
}

impl<K: PartialEq, V> PartialEq for Pair<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<K: Eq, V> Eq for Pair<K, V> {}

impl<K: PartialOrd, V> PartialOrd for Pair<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

impl<K: Ord, V> Ord for Pair<K, V> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}

#[inline]
pub(crate) fn find_longest_path(map: &Map) -> usize {
    let start = State::new(map.start);
    let mut q = BinaryHeap::new();
    q.push(Pair::new(start.heur(map), start));
    let mut longest = 0;

    while let Some(pair) = q.pop() {
        let mut value = pair.value;

        // Loop while there is a single possible move.  It is cheaper to not to insert it
        // into the queue.
        loop {
            if value.last == map.end {
                // A* heuristic doesn't work properly, so use really max value.
                if longest < value.path.len() {
                    longest = value.path.len();
                    eprintln!("new longest: {longest}");
                }
            }

            let mut around: arrayvec::ArrayVec<_, 4> = map
                .around(value.last)
                .filter_map(|p| value.try_extend(p))
                .collect();
            if around.len() == 1 {
                value = around.pop().unwrap();
                continue;
            } else {
                for n in around {
                    q.push(Pair::new(n.heur(map), n));
                }
                break;
            }
        }
    }

    return longest;
}
