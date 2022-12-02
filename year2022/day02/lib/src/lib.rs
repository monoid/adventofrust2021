use std::{io::BufRead, str::FromStr};

use strum::EnumString;

#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumString)]
pub enum Move {
    #[strum(serialize = "A", serialize = "X")]
    Rock = 1,
    #[strum(serialize = "B", serialize = "Y")]
    Paper = 2,
    #[strum(serialize = "C", serialize = "Z")]
    Scissors = 3,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumString)]
#[repr(i32)]
pub enum Outcome {
    #[strum(serialize = "X")]
    Loose,
    #[strum(serialize = "Y")]
    Draw,
    #[strum(serialize = "Z")]
    Win,
}

pub fn score(opp: Move, our: Move) -> i32 {
    use Move::*;

    let outcome: i32 = match (opp, our) {
        // draw
        (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3,
        // won
        (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => 6,
        // loose
        (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => 0,
    };
    outcome + (our as i32)
}

pub fn move_by_outcome(opp: Move, out: Outcome) -> Move {
    use Move::*;
    use Outcome::*;

    match (opp, out) {
        (_, Draw) => opp,

        (Rock, Loose) => Scissors,
        (Rock, Win) => Paper,
        (Paper, Loose) => Rock,
        (Paper, Win) => Scissors,
        (Scissors, Loose) => Paper,
        (Scissors, Win) => Rock,
    }
}

fn parse_line<S: FromStr>(inp: &str) -> (Move, S)
where
    <S as FromStr>::Err: std::fmt::Debug,
{
    let (opp, our) = inp.split_once(' ').unwrap();
    (Move::from_str(opp).unwrap(), S::from_str(our).unwrap())
}

pub fn read_script<S: FromStr>() -> Vec<(Move, S)>
where
    <S as FromStr>::Err: std::fmt::Debug,
{
    let inp = std::io::stdin();
    let inp = inp.lock();
    inp.lines().map(|s| parse_line(&s.unwrap())).collect()
}

#[cfg(test)]
mod tests {
    use crate::{score, Move, Our};

    #[test]
    fn test_score() {
        assert_eq!(score(Move::Rock, Our::Y), 8);
        assert_eq!(score(Move::Paper, Our::X), 1);
        assert_eq!(score(Move::Scissors, Our::Z), 6);
    }
}
