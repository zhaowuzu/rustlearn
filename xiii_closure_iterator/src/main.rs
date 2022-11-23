use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
}

fn simulated_expensive_calculation(intensity:u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}