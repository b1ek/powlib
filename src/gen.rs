
use rand::random;

use crate::{hash::hash_num, num::Num};

#[derive(Debug, Clone)]
pub struct POWRange {
    pub min: u128,
    pub max: u128
}

impl POWRange {
    pub fn new(min: u128, max: u128) -> POWRange {
        POWRange { min, max }
    }
}

#[derive(Debug, Clone)]
pub struct POWChallenge {
    pub hash: [u8; 32],
    pub range: POWRange
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

    /**
       Solve a chunk of numbers.
       
       This method is deprecated. Use `POWSolver` instead.
     */
    #[deprecated]
    pub fn chunk_solve(self: &POWChallenge, start: u128, end: u128) -> Option<u128> {
        for i in start..end {
            if self.check(Num::new(i)) {
                return Some(i)
            }
        }
        None
    }

    /**
       Solve the challenge.
       
       This method is deprecated. Use `POWSolver` instead.
     */
    #[deprecated]
    pub fn solve_singlethread(self: &POWChallenge) -> u128 {
        // the method is deprecated itself
        
        #[allow(deprecated)]
        match self.chunk_solve(self.range.min, self.range.max) {
            Some(v) => v,
            None => panic!("Number not found in range")
        }
    }
}
