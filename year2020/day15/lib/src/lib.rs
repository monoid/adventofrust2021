#[derive(Debug, Default)]
pub struct Game {
    pub history: Vec<i32>,
    steps: i32,
}

impl Game {
    pub fn new(size: usize) -> Self {
        Self {
            history: vec![-1; size],
            steps: 0,
        }
    }

    pub fn turn(&mut self, num: i32) -> i32 {
        let res = if self.history[num as usize] != -1 {
            let res = self.steps - self.history[num as usize];
            res
        } else {
            0
        };
        self.history[num as usize] = self.steps;
        self.steps += 1;
        res
    }
}
