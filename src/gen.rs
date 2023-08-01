
use rand::random;

use crate::{hash::hash_num, num::Num};

#[derive(Debug, Clone)]
pub struct POWRange {
    min: u128,
    max: u128
}

impl POWRange {
    pub fn new(min: u128, max: u128) -> POWRange {
        POWRange { min, max }
    }
}

pub struct POWSolver {
    challenge: POWChallenge
}

impl POWSolver {

    /**
     * Solve chunk with feedback.  
     * Feedback is a function that is being
     * called at each iteration of a loop,
     * which will be useful for progressbars
     * and visual feedback.
     * 
     * Example:
     * ```rust
       let challenge = POWChallenge::make(POWRange::new(0, 16));
       let num = POWSolver::new(challenge).chunk_solve_feedback(0, 16, |x| println!("{}", x)).unwrap();
       println!("Found number: {}", num)
     * ```
     */
    pub fn chunk_solve_feedback(self: &POWSolver, start: u128, end: u128, callback: fn(u128)) -> Option<u128> {
        for i in start..end {
            callback(i);
            if self.challenge.check(i.into()) {
                return Some(i);
            }
        }
        None
    }

    pub fn new(challenge: POWChallenge) -> POWSolver {
        POWSolver { challenge }
    }
}

#[derive(Debug, Clone)]
pub struct POWChallenge {
    hash: [u8; 32],
    range: POWRange
}

impl POWChallenge {
    pub fn new(hash: [u8; 32], range: POWRange) -> POWChallenge {
        POWChallenge { hash, range }
    }

    pub fn make(range: POWRange) -> POWChallenge {
        let secret = (random::<u128>() % (range.max - range.min)) + range.min;
        let secret = Num::new(secret);
        POWChallenge { hash: hash_num(secret), range: range }
    }

    pub fn make_from(num: Num, range: Option<POWRange>) -> POWChallenge {
        POWChallenge { hash: hash_num(num), range: match range {
            Some(v) => v,
            None => POWRange { min: 0, max: num * 2 }
        } }
    }

    pub fn check(self: &POWChallenge, num: Num) -> bool {
        hash_num(num) == self.hash
    }

    pub fn chunk_solve(self: &POWChallenge, start: u128, end: u128) -> Option<u128> {
        for i in start..end {
            if self.check(Num::new(i)) {
                return Some(i)
            }
        }
        None
    }

    pub fn solve_singlethread(self: &POWChallenge) -> u128 {
        match self.chunk_solve(self.range.min, self.range.max) {
            Some(v) => v,
            None => panic!("Number not found in range")
        }
    }
}
