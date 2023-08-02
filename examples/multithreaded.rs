
use powlib::{gen::{POWRange, POWChallenge}, solver::POWSolver};

fn main() {
    let challenge = POWChallenge::make(POWRange::new(0, 20480));
    let mut solver = POWSolver::new(challenge);
    println!("Found {} with 8 threads", solver.solve_blocking(8, Some(|x| println!("{x}"))));
}