use std::collections::HashMap;

const SCORES: [u8; 27] = [
    3, 4, 5,
    4, 5, 6,
    5, 6, 7,

    4, 5, 6,
    5, 6, 7,
    6, 7, 8,

    5, 6, 7,
    6, 7, 8,
    7, 8, 9,
];

type State = HashMap<((u8, u8), (u8, u8)), u64>;

#[derive(Debug, Copy, Clone)]
enum Player {
    First,
    Second,
}

impl Player {
    fn tuple_mut<A>(self, data: &mut (A, A)) -> &mut A {
        match self {
            Player::First => &mut data.0,
            Player::Second => &mut data.1,
        }
    }

    fn tuple_ref<A>(self, data: &(A, A)) -> &A {
        match self {
            Player::First => &data.0,
            Player::Second => &data.1,
        }
    }

    fn other(self) -> Self {
        match self {
            Player::First => Player::Second,
            Player::Second => Player::First,
        }
    }
}

fn move_pawn(space: u8, score: u8) -> u8 {
    ((space - 1) + score) % 10 + 1
}

/// Return next state, excluding the won games, and count of won games
fn advance(state: State, player: Player, limit: u8) -> (State, (u64, u64)) {
    let mut won = (0, 0);
    let mut next = State::new();

    for (&st, &cnt) in state.iter() {
        for &score in SCORES.iter() {
            let mut st = st;
            let new_pos = move_pawn(player.tuple_ref(&st).0, score);
            let new_score = player.tuple_ref(&st).1 + new_pos;
            if new_score >= limit {
                *player.tuple_mut(&mut won) += cnt;
            } else {
                *player.tuple_mut(&mut st) = (new_pos, new_score);
                *next.entry(st).or_default() += cnt;
            }
        }
    }
    (next, won)
}

fn main() {
    // let mut state = HashMap::from_iter([(((4, 0), (8, 0)), 1)].into_iter());
    let mut state = HashMap::from_iter([(((8, 0), (5, 0)), 1)].into_iter());
    let mut won_total = (0, 0);
    let mut player = Player::First;
    const LIMIT: u8 = 21;

    while !state.is_empty() {
        let (next, won) = advance(state, player, LIMIT);
        state = next;
        won_total.0 += won.0;
        won_total.1 += won.1;
        player = player.other();
    }

    println!("{}", std::cmp::max(won_total.0, won_total.1));
}
