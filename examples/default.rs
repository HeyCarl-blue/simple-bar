use simple_bar::ProgressBar;
use std::{thread::sleep, time::Duration};

fn main() {
    let num_iterations = 500;

    let mut bar = ProgressBar::default(num_iterations);

    for _ in 0..num_iterations {
        bar.next();
        sleep(Duration::from_millis(100));
    }
}