pub struct Race {
    pub time: u64,
    pub max_distance: u64,
}

impl Race {
    #[inline]
    pub fn distance(&self, push_time: u64) -> u64 {
        let speed = push_time;
        let time_left = self.time.checked_sub(push_time).unwrap();

        speed.checked_mul(time_left).unwrap()
    }

    pub fn beats(&self) -> u64 {
        // equation: x*(b-x) = c, or x*x - b*x + c = 0
        // where b -- time
        //       c -- max_distance
        // max at b/2
        // D = b*b - 4c
        // x1,x2 = (-b +/- sqrt(D))/2
        let b = self.time as f64;
        let c = self.max_distance as f64;
        let d = b * b - 4.0 * c;
        let sd = d.sqrt();
        let x1 = (b - sd) / 2.0;
        let x2 = (b + sd) / 2.0;
        let n1 = x1.ceil() as u64;
        let n2 = x2.floor() as u64;
        (n2 - n1) + 1
    }
}

pub const RACES_REAL: [Race; 4] = [
    Race {
        time: 40,
        max_distance: 215,
    },
    Race {
        time: 92,
        max_distance: 1064,
    },
    Race {
        time: 97,
        max_distance: 1505,
    },
    Race {
        time: 90,
        max_distance: 1100,
    },
];

pub const RACES_EXAMPLE: [Race; 3] = [
    Race {
        time: 7,
        max_distance: 9,
    },
    Race {
        time: 15,
        max_distance: 40,
    },
    Race {
        time: 30,
        max_distance: 200,
    },
];

pub const RACE2: Race = Race {
    time: 40_92_97_90,
    max_distance: 215_1064_1505_1100,
};
