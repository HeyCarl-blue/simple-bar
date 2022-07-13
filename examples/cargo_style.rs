use simple_bar::ProgressBar;
use std::{thread::sleep, time::Duration};

fn main() {
    let num_iterations = 500;

    let mut bar = ProgressBar::cargo_style(num_iterations, 50);

    for _ in 0..num_iterations {
        bar.next();
        sleep(Duration::from_millis(10));
    }
}