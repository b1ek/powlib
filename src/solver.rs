use std::{sync::{mpsc, Arc, atomic::{AtomicBool, Ordering}}, thread::{self, JoinHandle}};

use crate::{gen::POWChallenge, num::Num};

#[cfg(feature = "tokio_futures")]
use {tokio, tokio::task};

#[derive(Debug, Clone)]
pub struct POWSolver {
    challenge: POWChallenge,
    result: Option<u128>
}

enum SolverThreadMessage {
    PlainNumber(u128),
    Found(u128)
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
    pub fn solve_blocking(self: &mut POWSolver, threads: u8, mut callback: Option<impl FnMut(u128)>) -> u128 {
        let (send, recv) = mpsc::sync_channel::<SolverThreadMessage>(1);
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

                        if solver.challenge.check(Num::from(j as u128)) {
                            stop.store(true, Ordering::Relaxed);
                            send.send(SolverThreadMessage::Found(j)).unwrap();
                        } else {
                            send.send(SolverThreadMessage::PlainNumber(j)).unwrap();
                        }
                    }
                })
            );

            thread_start = thread_start + size + 1;
        }

        let mut result = 0;

        loop {
            let status = recv.recv().unwrap();
            match status {
                SolverThreadMessage::PlainNumber(num) => {
                    if callback.is_some() {
                        callback.as_mut().unwrap()(num);
                    }
                },
                SolverThreadMessage::Found(num) => {
                    result = num;
                    stop.store(true, Ordering::Relaxed);
                    break;
                }
            }

            if stop.load(Ordering::Relaxed) {
                let mut all_finished = true;
                for handle in handles.iter() {
                    if !handle.is_finished() {
                        all_finished = false
                    }
                }
                if all_finished { break }
            }
        }

        self.result = Some(result);

        result
    }

    #[cfg(feature = "tokio_futures")]
    pub async fn chunk_future(self: &POWSolver, start: u128, end: u128) -> Option<u128> {
        for i in start..end {
            if self.challenge.check(Num::from(i)) {
                return Some(i);
            }
        }
        None
    }
    
    #[cfg(feature = "tokio_futures")]
    pub async fn solve_futures(self: &mut POWSolver, futures: u8) -> Result<u128, String> {
        use tokio::task::JoinSet;


        let mut start: u128 = 0;
        let size = self.chunksize(futures);
        let range = self.challenge.range.max - self.challenge.range.min;

        let stop = Arc::new(AtomicBool::new(false));

        let mut tasks: JoinSet<Option<u128>> = JoinSet::new();
        for _ in 0..futures {
            let mut end = start + size;
            let stop = stop.clone();
            let solver = self.clone();

            if end > range {
                end -= end % range;
            }

            tasks.spawn(async move {
                for i in 0..end {
                    if stop.load(Ordering::Relaxed) {
                        break;
                    }
                    if solver.challenge.check(Num::from(i)) {
                        stop.store(true, Ordering::Relaxed);
                        return Some(i);
                    }
                }
                None
            });

            start = start + size + 1;
        }

        for _ in 0..futures {
            let result = tasks.join_next().await;
            if result.is_none() {
                return Err("Join set is empty".to_string());
            }
            let result = result.unwrap();

            if result.is_err() {
                return Err(format!("JoinError: {}", result.unwrap_err()));
            }

            let result = result.unwrap();

            if result.is_some() {
                self.result = Some(result.unwrap());
                return Ok(self.result.unwrap());
            }
        }
        Err("Failed to find".to_string())

    }

    pub fn new(challenge: POWChallenge) -> POWSolver {
        POWSolver { challenge, result: None }
    }
}