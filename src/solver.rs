use std::{sync::{mpsc, Arc, atomic::{AtomicBool, Ordering}}, thread::{self, JoinHandle}};

use crate::{gen::POWChallenge, num::Num};

#[derive(Debug, Clone)]
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

    pub fn chunksize(self: &POWSolver, threads: u8) -> u128 {
        let range = self.challenge.range.max - self.challenge.range.min;
        range / threads as u128
    }

    /**
       Solve a challenge using multiple threads,
       which would be much faster than using just one thread

       This method also supports callbacks via `Option`
     */
    pub fn solve_blocking(self: &mut POWSolver, threads: u8, callback: Option<fn(u128)>) -> u128 {
        let (send, recv) = mpsc::sync_channel::<u128>(1);
        let mut thread_start: u128 = self.challenge.range.min;

        let size = self.chunksize(threads);
        let mut handles: Vec<JoinHandle<()>> = vec![];
        let stop = Arc::new(AtomicBool::new(false));
        let range = self.challenge.range.max - self.challenge.range.min;

        for _ in 0..threads {
            let solver = self.clone();
            let send = send.clone();
            let stop = stop.clone();

            let mut end = thread_start + size;
            if end > range {
                end -= end % range;
            }

            handles.push(
                    thread::spawn(move || {
                        for j in thread_start..end {
                            if stop.load(Ordering::Relaxed) { break }
                            if callback.is_some() {
                                callback.unwrap()(j);
                            }

                            if solver.challenge.check(Num::from(j as u128)) {
                                stop.store(true, Ordering::Relaxed);
                                send.send(j as u128).unwrap();
                            }
                        }
                    }
                )
            );

            thread_start = thread_start + size + 1;
        }

        let result = recv.recv().unwrap();
        stop.load(Ordering::Relaxed);

        loop {
            let mut all_finished = true;
            for handle in handles.iter() {
                if !handle.is_finished() {
                    all_finished = false
                }
            }
            if all_finished { break }
        }

        self.result = Some(result);

        result
    }

    pub fn new(challenge: POWChallenge) -> POWSolver {
        POWSolver { challenge, result: None }
    }
}