# POWlib
POWlib is a pure rust library for scrypt-based proof-of-work challenges.  
It uses 32-byte long scrypt hash with following parameters: `log_n: 6, r: 64, p: 1`

# Example usage
To load an existing challenge: you may use `powlib::gen::POWChallenge { hash: [a 32-byte long hash], range: POWRange::new(min, max) }`  
To generate a challenge, use this: `powlib::gen::POWChallenge::make(POWRange::new(min, max))`  
To solve a challenge, use this: `challenge.solve_singlethread()`

Threaded solve method is planned but if you want to implement your own, there is a chunked method for solving: `challenge.chunk_solve(min, max)`

So, the simplest code would look like this:

```rust

use powlib::{self, gen::{POWRange, POWChallenge}};

fn main() {
    let challenge = POWChallenge::make(POWRange::new(0, 20480));
    println!("{}", challenge.solve_singlethread());
}

```

# Examples
To run examples that have their own directory, cd to their directory and run `cargo r`. Like this:

```sh
$ cd examples/multithreaded
$ cargo r --release # debug version will be much slower
```