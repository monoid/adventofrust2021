use std::io::BufRead;
use std::io::Read;
use std::str::FromStr;

pub type PageId = u32;

pub struct Rule {
    pub before: PageId,
    pub after: PageId,
}

impl FromStr for Rule {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (before, after) = s.split_once('|').ok_or(())?;
        let before = PageId::from_str(before).ok().ok_or(())?;
        let after = PageId::from_str(after).ok().ok_or(())?;
        Ok(Self { before, after })
    }
}

pub type Manual = Vec<PageId>;

pub struct Data {
    pub rules: Vec<Rule>,
    pub manuals: Vec<Manual>,
}

impl Data {
    pub fn read<R: Read>(inp: R) -> Self {
        let mut buf = std::io::BufReader::new(inp);

        let mut rules = vec![];
        let mut manuals = vec![];

        for line in (&mut buf).lines() {
            let line = line.unwrap();
            if line.is_empty() {
                break;
            }

            rules.push(Rule::from_str(&line).unwrap());
        }

        for line in buf.lines() {
            let line = line.unwrap();
            manuals.push(line.split(',').map(|s| s.parse().unwrap()).collect());
        }

        Self { rules, manuals }
    }
}
