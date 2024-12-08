use std::str::FromStr;

pub type N = i64;
pub struct Rule {
    pub target: N,
    pub values: Vec<N>,
}

impl Rule {
    pub fn solvable(&self) -> bool {
        let combinations = generate_bin_combinations::<Op1>(self.values.len() as u32 - 1);
        for comb in combinations {
            let mut values = self.values.iter().cloned();
            let mut acc = values.next().unwrap();

            for (op, val) in comb.zip(values) {
                acc = op.apply(acc, val);
            }
            if acc == self.target {
                return true;
            }
        }

        false
    }

    pub fn solvable2(&self) -> bool {
        // look from the tail; concat then is really a "suffix match", and mul is "division"
        let curr = self.target;
        let (&goal, vals) = self.values.split_first().expect("non-empty values");
        search2(curr, goal, vals)
    }
}

fn search2(curr: N, goal: N, vals: &[N]) -> bool {
    match vals.split_last() {
        Some((&val, rest_vals)) => {
            for op in [Op2::Plus, Op2::Mul, Op2::Concat] {
                if let Some(new_cur) = op.try_match(curr, val) {
                    if search2(new_cur, goal, rest_vals) {
                        return true
                    }
                }
            }
            false

        }
        None => curr == goal,
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (target_str, values_str) = s.split_once(": ").ok_or(())?;
        let target = target_str.parse().ok().ok_or(())?;
        let values = values_str
            .split(' ')
            .map(|s| s.parse::<i64>().ok().ok_or(()))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { target, values })
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Op1 {
    Plus,
    Mul,
}

impl Op1 {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Op1::Plus => a + b,
            Op1::Mul => a * b,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Op2 {
    Plus,
    Mul,
    Concat,
}

impl Op2 {
    fn try_match(self, target: N, n: N) -> Option<N> {
        match self {
            Op2::Plus => Some(target - n),
            Op2::Mul => {
                let md = target % n;
                if md == 0 {
                    Some(target / n)
                } else {
                    None
                }
            }
            Op2::Concat => {
                let nstr = n.to_string();
                let tstr = target.to_string();
                if let Some(v) = tstr.strip_suffix(&nstr) {
                    v.parse().ok()
                } else {
                    None
                }
            }
        }
    }
}

impl From<bool> for Op1 {
    fn from(b: bool) -> Self {
        if b {
            Op1::Plus
        } else {
            Op1::Mul
        }
    }
}

fn generate_bin_combinations<T: From<bool>>(
    size: u32,
) -> impl Iterator<Item = impl Iterator<Item = T>> {
    (0..(1 << size)).map(move |n| (0..size).map(move |offset| ((1 & (n >> offset)) != 0).into()))
}
