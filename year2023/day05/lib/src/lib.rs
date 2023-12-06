use std::{ops::RangeInclusive, str::FromStr};

pub type Seed = u64;

#[derive(Debug)]
pub struct Rule {
    pub target: Seed,
    pub source: Seed,
    pub len: Seed,
}

impl Rule {
    pub fn transform(&self, input: Seed) -> Option<Seed> {
        if (self.source..(self.source + self.len)).contains(&input) {
            Some(self.target + (input - self.source))
        } else {
            None
        }
    }
}

impl FromStr for Rule {
    type Err = nom::Err<()>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use nom::character::complete::space1;
        use nom::character::complete::u64;
        use nom::combinator::{all_consuming, map};
        use nom::sequence::separated_pair;

        all_consuming(map(
            separated_pair(u64, space1, separated_pair(u64, space1, u64)),
            |(target, (source, len))| Self {
                target,
                source,
                len,
            },
        ))(s.trim())
        .map(|(_, val)| val)
    }
}

#[derive(Debug)]
pub struct Map {
    pub rules: Vec<Rule>,
}

impl Map {
    pub fn transform(&self, value: Seed) -> Seed {
        self.rules
            .iter()
            .find_map(|rule| rule.transform(value))
            .unwrap_or(value)
    }

    pub fn transform_range(
        &self,
        mut value: RangeInclusive<Seed>,
        target: &mut Vec<RangeInclusive<Seed>>,
    ) {
        assert!(!value.is_empty());
        let rules = self.rules.iter();
        // N.B.: rules is sorted by source.
        let initial_start = *value.start();
        let rules = rules.skip_while(|r| (r.source + r.len) < initial_start);
        // if value falls between the rules, the for is not executed, and the range is returned as is.
        let mut inserted = false;
        let value_end = *value.end();
        for rule in rules.take_while(|r| value_end >= r.source) {
            inserted = true;
            // do the dzhob

            // value initial value may be smaller than next rule
            if value.start() < &rule.source {
                assert!(&rule.source <= value.end(), "{:?} vs {:?}", rule, value);
                // split the interval into two ones
                let prev = *value.start()..=rule.source - 1;
                value = rule.source..=*value.end();
                target.push(prev);
            }

            assert!(&rule.source <= value.start());

            let new_value_start = rule
                .transform(*value.start())
                .expect("start is within rule range");
            if *value.end() < (rule.source + rule.len) {
                // the rule consumes all the value
                let new_value_end = rule
                    .transform(*value.end())
                    .expect("end is within rule range");
                target.push(new_value_start..=new_value_end);

                break;
            } else {
                let new_value_end = rule.target + rule.len - 1;
                target.push(new_value_start..=new_value_end);

                value = rule.source + rule.len..=*value.end();
            }
        }

        if !inserted {
            target.push(value);
        }
    }

    pub fn transform_ranges(&self, ranges: &[RangeInclusive<Seed>]) -> Vec<RangeInclusive<Seed>> {
        let mut result = vec![];
        for range in ranges {
            self.transform_range(range.clone(), &mut result);
        }
        result
    }

    pub fn read_map<R: Iterator<Item = String>>(input: R) -> Map {
        let mut rules: Vec<_> = input.map(|line| Rule::from_str(&line).unwrap()).collect();
        rules.sort_unstable_by_key(|rule| rule.source);
        Map { rules }
    }
}

fn parse_seeds(inp: &str) -> Vec<Seed> {
    let inp = inp.strip_prefix("seeds: ").expect("no prefix");
    inp.split_ascii_whitespace()
        .map(|tok| tok.parse().unwrap())
        .collect()
}

pub fn read_scene() -> (Vec<Seed>, Vec<Map>) {
    let mut lines = std::io::stdin().lines();

    let seeds_str = lines.next().unwrap().unwrap();
    let seeds = parse_seeds(&seeds_str);

    // empty
    lines.next();

    let mut maps = vec![];

    loop {
        let header = lines.next();
        match header {
            None => return (seeds, maps),
            Some(_) => {
                // we ignore header
                maps.push(Map::read_map(
                    (&mut lines)
                        .map(|l| l.unwrap())
                        .take_while(|l| !l.trim().is_empty()),
                ));
            }
        }
    }
}
