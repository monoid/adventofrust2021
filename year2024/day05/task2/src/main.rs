use std::collections::{HashMap, HashSet};

fn main() {
    let mut data = lib::Data::read(std::io::stdin());
    let mut sum = 0;
    for man in &mut data.manuals {
        for rule in &data.rules {
            let before = man.iter().position(|n| n == &rule.before);
            let after = man.iter().position(|n| n == &rule.after);
            if !before
                .zip(after)
                .map(|(before_idx, after_idx)| before_idx < after_idx)
                .unwrap_or(true)
            {
                topo_sort(&mut *man, &data.rules);
                sum += man[man.len() / 2];
                break;
            }
        }
    }
    println!("{sum}");
}

fn topo_sort(data: &mut [lib::PageId], rules: &[lib::Rule]) {
    let real: HashSet<lib::PageId> = data.iter().cloned().collect();

    let mut before_ordering = HashMap::<lib::PageId, HashSet<lib::PageId>>::new();
    let mut after_ordering = HashMap::<lib::PageId, Vec<lib::PageId>>::new();
    for rule in rules {
        if real.contains(&rule.before) && real.contains(&rule.after) {
            before_ordering
                .entry(rule.after)
                .or_default()
                .insert(rule.before);
            after_ordering
                .entry(rule.before)
                .or_default()
                .push(rule.after);
        }
    }

    let mut free_bag = vec![];
    for item in &*data {
        if !before_ordering.contains_key(item) {
            free_bag.push(*item);
        }
    }

    assert!(!free_bag.is_empty(), "Cyclic dependencies");
    if free_bag.len() > 1 {
        dbg!(&free_bag);
    }

    let mut output = vec![];

    while let Some(next_free) = free_bag.pop() {
        output.push(next_free);
        free_parent(
            next_free,
            &mut before_ordering,
            &after_ordering,
            &mut free_bag,
        );
        if free_bag.len() > 1 {
            dbg!(&free_bag);
        }
    }

    assert_eq!(output.len(), data.len());
    data.copy_from_slice(&output);
}

fn free_parent(
    id: lib::PageId,
    before_ordering: &mut HashMap<lib::PageId, HashSet<lib::PageId>>,
    after_ordering: &HashMap<lib::PageId, Vec<lib::PageId>>,
    free_bag: &mut Vec<lib::PageId>,
) {
    if let Some(children) = after_ordering.get(&id) {
        for child in children.iter().cloned() {
            if let Some(parents) = before_ordering.get_mut(&child) {
                parents.remove(&id);
                if parents.is_empty() {
                    free_bag.push(child);
                }
            }
        }
    }
}
