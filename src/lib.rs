#![no_std]

pub struct Cabezita {
    tick: u16,
    tick_limit: u16,
    iteration: u16,
}

impl Cabezita {
    pub fn new() -> Cabezita {
        Cabezita {
            tick: 0,
            tick_limit: 5_000, 
            iteration: 1_000,
        }
    }

    pub fn increment(&mut self) -> u16 {
        self.tick = (self.tick + self.iteration) % self.tick_limit;

        self.tick
    }
}
