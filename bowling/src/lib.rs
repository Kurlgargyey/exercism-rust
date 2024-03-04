#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    score: u16,
    frame: usize,
    pins_in_frame: u16,
    throws_in_frame: usize,
    doubling_throws: Vec<usize>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            score: 0,
            frame: 0,
            pins_in_frame: 10,
            throws_in_frame: 2,
            doubling_throws: Vec::<usize>::new(),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        self.validate_throw(pins)?;
        self.score_throw(pins);
        self.calculate_doubling_throws(pins);
        self.calculate_frame(pins);
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.game_is_complete() {
            return Some(self.score);
        }
        println!(
            "Game has not finished yet! We are at frame {:?} and the score is {:?}.",
            self.frame,
            self.score
        );
        None
    }

    fn game_is_complete(&self) -> bool {
        self.past_last_frame() && self.doubling_throws.is_empty()
    }

    fn past_last_frame(&self) -> bool {
        self.frame >= 10
    }

    fn validate_throw(&self, pins: u16) -> Result<(), Error> {
        if self.game_is_complete() {
            return Err(Error::GameComplete);
        }
        if pins > self.pins_in_frame {
            return Err(Error::NotEnoughPinsLeft);
        }
        Ok(())
    }

    fn score_throw(&mut self, pins: u16) {
        if !self.past_last_frame() {
            self.score += pins;
        }
        self.doubling_throws = self.doubling_throws
            .iter()
            .map(|throw_count| {
                self.score += pins;
                *throw_count - 1
            })
            .filter(|throw_count| *throw_count > 0)
            .collect();
    }

    fn calculate_doubling_throws(&mut self, pins: u16) {
        if self.past_last_frame() {
            return;
        }
        if pins == self.pins_in_frame {
            match self.throws_in_frame {
                2 => self.doubling_throws.push(2),
                _ => self.doubling_throws.push(1),
            }
        }
    }

    fn calculate_frame(&mut self, pins: u16) {
        self.pins_in_frame -= pins;
        self.throws_in_frame -= 1;
        if self.pins_in_frame == 0 || self.throws_in_frame == 0 {
            self.reset_frame();
        }
    }

    fn reset_frame(&mut self) {
        self.frame += 1;
        self.pins_in_frame = 10;
        self.throws_in_frame = 2;
    }
}
