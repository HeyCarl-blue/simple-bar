use std::time::Duration;
use std::thread;

use simple_bar::ProgressBar;

fn main() {
    let num_iterations = 1000;
    let mut bar = ProgressBar::default_eta(num_iterations, 50);

    for _ in 0..num_iterations {
        bar.next();
        thread::sleep(Duration::from_millis(100));
    }
}