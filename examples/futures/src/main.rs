use powlib::{gen::{POWChallenge, POWRange}, solver::POWSolver};


#[tokio::main]
async fn main() {
    let challenge = POWChallenge::make(POWRange::new(0, 256));
    let mut solver = POWSolver::new(challenge);
    let res = solver.solve_futures((num_cpus::get() as u8) * 2).await.unwrap();
    println!("{}", res);
}
