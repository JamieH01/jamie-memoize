#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use std::collections::HashMap;
    use std::thread;
    use std::time::{Duration, Instant};
    use lazy_static::lazy_static;

    extern crate memo;
    use memo::memoize;

    #[memoize]
    fn slow_fn(a: u32) -> u32 {
        thread::sleep(Duration::from_secs(5));
        a * 2
    }

    #[test]
    fn memo_test() {
        let time = Instant::now();
        println!("slow_fn(5) = {}\ntime to exec: {:?}", slow_fn(5), time.elapsed());

        let time = Instant::now();
        println!("slow_fn(5) = {}\ntime to exec: {:?}", slow_fn(5), time.elapsed());
    }

}
