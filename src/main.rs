use std::thread::sleep;
use std::time::Duration;

const CLEAR: &str = "\x1B[2J\x1b[1;1H";

fn main() {
    let v = vec![1, 2, 3];
    let mut i = 1;
    for n in v.iter() {
        println!("{}{}", CLEAR, "*".repeat(i));
        i += 1;
        expensive_calculation(n)
    }
}

fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_secs(1));
}
