My own memoization attribute macro. This is a solved issue, so im not posting this to crates.io, but leaving it here as a proof of concept 

```rust
use memo::memoize;

#[memoize]
fn slow_fn(a: u32) -> u32 {
    thread::sleep(Duration::from_secs(5));
    a * 2
}

fn main() {
    let time = Instant::now();
    println!("slow_fn(5) = {}\ntime to exec: {:?}", slow_fn(5), time.elapsed());

    let time = Instant::now();
    println!("slow_fn(5) = {}\ntime to exec: {:?}", slow_fn(5), time.elapsed());
}
```
