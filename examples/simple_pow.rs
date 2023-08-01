
use powlib::{self, gen::{POWRange, POWChallenge}};

fn main() {
    let challenge = POWChallenge::make(POWRange::new(0, 20480));
    println!("{}", challenge.solve_singlethread());
}