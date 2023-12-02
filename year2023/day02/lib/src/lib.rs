use maplit::hashmap;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::u32 as u32_parser,
    combinator::{all_consuming, map, value},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};
use std::collections::HashMap;

pub fn situation() -> HashMap<&'static str, u32> {
    hashmap! {
        "red" => 12,
        "green" => 13,
        "blue" => 14,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Game {
    pub id: u32,
    pub pulls: Vec<HashMap<&'static str, u32>>,
}

impl Game {
    pub fn is_possible(&self, situation: &HashMap<&'static str, u32>) -> bool {
        for pull in &self.pulls {
            for (color, num) in situation {
                if pull.get(color).cloned().unwrap_or(0) > *num {
                    return false;
                }
            }
        }
        true
    }

    pub fn min_situation(&self) -> HashMap<&'static str, u32> {
        let mut result = HashMap::default();

        for pull in &self.pulls {
            for (color, num) in pull {
                let max = std::cmp::max(result.get(color).cloned().unwrap_or_default(), *num);
                result.insert(*color, max);
            }
        }

        result
    }
}

pub fn parse_game(inp: &str) -> IResult<&str, Game> {
    all_consuming(map(
        separated_pair(
            preceded(tag("Game "), u32_parser),
            tag(": "),
            separated_list1(tag("; "), parse_pulls),
        ),
        |(id, pulls)| Game { id, pulls },
    ))(inp)
}

fn parse_pulls(inp: &str) -> IResult<&str, HashMap<&'static str, u32>> {
    map(separated_list1(tag(", "), parse_pull), |vec| {
        vec.into_iter().collect()
    })(inp)
}

fn parse_pull(inp: &str) -> IResult<&str, (&'static str, u32)> {
    map(
        separated_pair(
            u32_parser,
            tag(" "),
            alt((
                value("green", tag("green")),
                value("red", tag("red")),
                value("blue", tag("blue")),
            )),
        ),
        |(n, col)| (col, n),
    )(inp)
}

#[cfg(test)]
mod tests {
    use crate::{parse_game, Game, SITUATION};
    use maplit::hashmap;

    #[test]
    fn test_parser() {
        let inp = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = parse_game(inp).unwrap().1;
        assert_eq!(
            game,
            Game {
                id: 1,
                pulls: vec![
                    hashmap! {
                        "blue" => 3,
                        "red" => 4,
                    },
                    hashmap! {
                        "red"=> 1,
                        "green"=> 2,
                        "blue"=> 6,
                    },
                    hashmap! {
                        "green"=> 2,
                    },
                ]
            }
        );
    }

    #[test]
    fn test_possible() {
        let inp = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = parse_game(inp).unwrap().1;

        assert!(game.is_possible(&SITUATION))
    }

    #[test]
    fn test_impossible() {
        let inp = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let game = parse_game(inp).unwrap().1;

        assert!(!game.is_possible(&SITUATION))
    }
}
