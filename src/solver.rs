use crate::gen::POWChallenge;

pub struct POWSolver {
    challenge: POWChallenge,
    result: Option<u128>
}

impl POWSolver {

    /**
       Solve chunk with feedback.  
       Feedback is a function that is
       being called at each iteration
       of a loop, which will be useful
       for progressbars and visual feedback.
       
       Example:
       ```rust
       let challenge = POWChallenge::make(POWRange::new(0, 16));
       let num = POWSolver::new(challenge).chunk_solve_feedback(0, 16, |x| println!("{}", x)).unwrap();
       println!("Found number: {}", num)
       ```
     */
    pub fn chunk_solve_feedback(self: &mut POWSolver, start: u128, end: u128, callback: fn(u128)) -> Option<u128> {
        for i in start..end {
            callback(i);
            if self.challenge.check(i.into()) {
                self.result = Some(i);
                return Some(i);
            }
        }
        None
    }

    /**
     * Solve a chunk.  
     * This function is the same as `POWRange.chunk_solve`, the only difference is that it doesnt call a function each iteration which could possibly save some time
     * 
     * Example:
     * ```rust
       let challenge = POWChallenge::make(POWRange::new(0, 16));
       let num = POWSolver::new(challenge).chunk_solve(0, 16).unwrap();
       println!("Found number: {}", num)
     * ```
     */
    pub fn chunk_solve(self: &mut POWSolver, start: u128, end: u128) -> Option<u128> {
        for i in start..end {
            if self.challenge.check(i.into()) {
                self.result = Some(i);
                return Some(i);
            }
        }
        None
    }

    /**
       Solve the challenge using just one thread.
     */
    pub fn solve_signle(self: &mut POWSolver) -> Option<u128> {
        return self.chunk_solve(self.challenge.range.min, self.challenge.range.max);
    }

    pub fn new(challenge: POWChallenge) -> POWSolver {
        POWSolver { challenge, result: None }
    }
}