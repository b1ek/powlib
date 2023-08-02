
use std::time::Instant;

use indicatif::ProgressBar;
use powlib::{gen::{POWRange, POWChallenge}, solver::POWSolver};

fn main() {
    let challenge = POWChallenge::make(POWRange::new(0, 204800));
    let mut solver = POWSolver::new(challenge);
    let mut count = 0;

    let progess = ProgressBar::new(204800);
    let thr_progress = progess.clone();

    let time = Instant::now();

    let result = solver.solve_blocking(16, Some(move |_| {
        thr_progress.set_position(count);
        count += 1;
    }));

    progess.finish();

    println!("Found {result} in {} ms", time.elapsed().as_secs_f32());
}