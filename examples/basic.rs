use mulstant::Mulstant;
use std::{thread, time::Duration};

fn main() {
    let mut mu = Mulstant::new();

    // Record events
    mu.record("initialization");
    thread::sleep(Duration::from_millis(500));

    mu.record("process_data");
    thread::sleep(Duration::from_millis(300));

    mu.record("cleanup");

    // Finalize and get results
    let result = mu.finalize();

    // Print summary
    println!("{}", result.summary());
}
