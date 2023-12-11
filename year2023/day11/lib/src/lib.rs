use std::collections::HashSet;

pub struct Univ {
    pub map: Vec<(usize, usize)>,
    pub has_galaxy_x: HashSet<usize>,
    pub has_galaxy_y: HashSet<usize>,
}

impl Univ {
    pub fn dist(&self, mut gal1: (usize, usize), mut gal2: (usize, usize), coef: u64) -> u64 {
        if gal1.0 > gal2.0 {
            std::mem::swap(&mut gal1.0, &mut gal2.0);
        }
        if gal1.1 > gal2.1 {
            std::mem::swap(&mut gal1.1, &mut gal2.1);
        }

        let mut exp = 0;

        for x in gal1.0 + 1..gal2.0 {
            exp += (!self.has_galaxy_x.contains(&x)) as u64 * coef;
        }

        for y in gal1.1 + 1..gal2.1 {
            exp += (!self.has_galaxy_y.contains(&y)) as u64 * coef;
        }

        (gal2.0 - gal1.0) as u64 + (gal2.1 - gal1.1) as u64 + exp
    }
}

pub fn read_data() -> Univ {
    let mut map = vec![];
    for (y, line) in std::io::stdin().lines().enumerate() {
        let line = line.unwrap();
        let line = line.trim();
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                map.push((x, y));
            }
        }
    }

    let mut has_galaxy_x = HashSet::new();
    let mut has_galaxy_y = HashSet::new();

    for (x, y) in &map {
        has_galaxy_x.insert(*x);
        has_galaxy_y.insert(*y);
    }

    Univ {
        map,
        has_galaxy_x,
        has_galaxy_y,
    }
}
