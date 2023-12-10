use std::collections::HashSet;

use lib::Tile;

fn main() {
    let map = lib::read_data();
    let start = lib::find_start_pos(&map).unwrap();
    let trace = lib::trace(start, &map);

    let inside = count_inside(&map, &trace);
    println!("{:?}", inside);
    eprintln!("trace len: {} of {}", trace.len(), map.len() * map[0].len());
}

#[derive(Debug, Copy, Clone)]
enum ScanState {
    // Outside means hear "outside of a line".
    Outside,
    FromBelow,
    FromAbove,
}

impl ScanState {
    fn update(self, tile: lib::Tile) -> (ScanState, bool) {
        match self {
            ScanState::Outside => Self::update_outside(tile),
            ScanState::FromBelow => Self::update_from_below(tile),
            ScanState::FromAbove => Self::update_from_above(tile),
        }
    }

    fn update_outside(tile: lib::Tile) -> (ScanState, bool) {
        match tile {
            Tile::Grnd => (ScanState::Outside, false),
            Tile::Vert => (ScanState::Outside, true),
            Tile::NeL => (ScanState::FromBelow, false),
            Tile::SeF => (ScanState::FromAbove, false),
            Tile::NwJ | Tile::Start | Tile::Sw7 | Tile::Hor => panic!("can't happen"),
        }
    }

    fn update_from_above(tile: lib::Tile) -> (ScanState, bool) {
        match tile {
            Tile::Grnd | Tile::Vert | Tile::NeL | Tile::SeF => panic!("can't happen"),
            Tile::Hor => (ScanState::FromAbove, false),
            Tile::NwJ => (ScanState::Outside, true),
            Tile::Sw7 | Tile::Start => (ScanState::Outside, false),
        }
    }

    fn update_from_below(tile: lib::Tile) -> (ScanState, bool) {
        match tile {
            Tile::Grnd | Tile::Vert | Tile::NeL | Tile::SeF => panic!("can't happen"),
            Tile::Hor => (ScanState::FromBelow, false),
            Tile::NwJ => (ScanState::Outside, false),
            Tile::Sw7 | Tile::Start => (ScanState::Outside, true),
        }
    }
}

fn count_inside(map: &[Vec<lib::Tile>], trace: &[lib::Pos]) -> usize {
    use std::io::Write;
    use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    let mut count = 0;

    let border = HashSet::<_>::from_iter(trace.iter().cloned());

    for (y, line) in map.iter().enumerate() {
        let mut inside = false;
        let mut scan_state = ScanState::Outside;

        for (x, tile) in line.iter().cloned().enumerate() {
            let contains = border.contains(&(x, y));
            let tile = if contains { tile } else { Tile::Grnd };

            let (new_scan_state, detected) = scan_state.update(tile);
            scan_state = new_scan_state;
            if detected {
                inside = !inside;
            }
            if inside && tile == lib::Tile::Grnd {
                count += 1;
                stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
                    .unwrap();
                write!(stdout, "#").unwrap();
                stdout.set_color(ColorSpec::new().set_fg(None)).unwrap();
            } else if contains {
                stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
                    .unwrap();
                write!(stdout, "{}", tile.char()).unwrap();
                stdout.set_color(ColorSpec::new().set_fg(None)).unwrap();
            } else {
                write!(stdout, "{}", tile.char()).unwrap();
            }
        }
        assert!(!inside);
        writeln!(stdout).unwrap();
    }
    count
}
