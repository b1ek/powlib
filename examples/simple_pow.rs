
use std::time::Instant;

use powlib::{self, gen::{POWRange, POWChallenge}, hash::hash_num, num::Num};

fn main() {
    {
        let num = Num::new(473823);
        let time = Instant::now();
        hash_num(num);
        println!("Simple hash of {} took {} millis", u128::from(num), time.elapsed().as_millis());
    }
    {
        let time = Instant::now();
        let challenge = POWChallenge::make(POWRange::new(0, 20480));
        println!("Solvling {} took {} seconds", challenge.solve_singlethread(), time.elapsed().as_secs_f32());
    }
}