use std::{collections::HashMap, ops::Range};

fn main() {
    let (wfs, _) = lib::read_data();
    let start = maplit::hashmap! {
        'a' => 1..4001,
        'm' => 1..4001,
        's' => 1..4001,
        'x' => 1..4001,
    };

    let cnt = count(&wfs, "in", &start);
    println!("{cnt}");
}

type ParamRange = HashMap<char, Range<u32>>;

fn count(workflows: &HashMap<String, lib::Workflow>, node: &str, start: &ParamRange) -> u64 {
    match node {
        "A" => start.values().map(|x| x.len() as u64).product(),
        "R" => 0,
        _ => {
            let mut sum = 0u64;
            let mut right = start.clone();
            let wf = workflows.get(node).unwrap();

            for rule in &wf.rules {
                let (left1, right1) = split_range(&right, rule.cat, rule.ord, rule.val);
                if let Some(left) = left1 {
                    sum += count(workflows, &rule.target, &left);
                }
                match right1 {
                    Some(next_right) => right = next_right,
                    // There is no point of summing the remaining zeroes.
                    None => return sum,
                }
            }
            sum += count(workflows, &wf.else_workflow, &right);

            sum
        }
    }
}

fn split_range(
    range: &ParamRange,
    cat: char,
    cmp: char,
    mut val: u32,
) -> (Option<ParamRange>, Option<ParamRange>) {
    let r = range.get(&cat).unwrap().clone();

    if cmp == '>' {
        val += 1;
    }

    let mut left = if val < r.start {
        None
    } else {
        let mut f = range.clone();
        f.insert(cat, r.start..std::cmp::min(r.end, val));
        Some(f)
    };

    let mut right = if r.end <= val {
        None
    } else {
        let mut f = range.clone();
        f.insert(cat, std::cmp::max(r.start, val)..r.end);
        Some(f)
    };

    if cmp == '>' {
        std::mem::swap(&mut left, &mut right);
    }
    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left_bound() {
        let m = maplit::hashmap! {
            'a' => 18..42,
        };

        let (r1, r2) = split_range(&m, 'a', '<', 42);
        assert_eq!(r1, Some(m));
        assert_eq!(r2, None);
    }

    #[test]
    fn test_left() {
        let m = maplit::hashmap! {
            'a' => 18..42,
        };

        let (r1, r2) = split_range(&m, 'a', '<', 48);
        assert_eq!(r1, Some(m));
        assert_eq!(r2, None);
    }

    #[test]
    fn test_left_bound_rev() {
        let m = maplit::hashmap! {
            'a' => 18..42,
        };

        let (r1, r2) = split_range(&m, 'a', '>', 42);
        assert_eq!(r1, None);
        assert_eq!(r2, Some(m));
    }

    #[test]
    fn test_left_rev() {
        let m = maplit::hashmap! {
            'a' => 18..42,
        };

        let (r1, r2) = split_range(&m, 'a', '>', 48);
        assert_eq!(r1, None);
        assert_eq!(r2, Some(m));
    }

    #[test]
    fn test_mid_bound() {
        let m = maplit::hashmap! {
            'a' => 18..42,
        };

        let (r1, r2) = split_range(&m, 'a', '<', 32);
        assert_eq!(
            r1,
            Some(maplit::hashmap! {
                'a' => 18..32,
            })
        );
        assert_eq!(
            r2,
            Some(maplit::hashmap! {
                'a' => 32..42,
            })
        );
    }

    #[test]
    fn test_mid_bound_rev() {
        let m = maplit::hashmap! {
            'a' => 18..42,
        };

        let (r1, r2) = split_range(&m, 'a', '>', 32);
        assert_eq!(
            r1,
            Some(maplit::hashmap! {
                'a' => 33..42,
            })
        );
        assert_eq!(
            r2,
            Some(maplit::hashmap! {
                'a' => 18..33,
            })
        );
    }
}
