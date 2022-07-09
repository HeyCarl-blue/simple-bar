#[test]
fn example_test() {
    use std::{thread::sleep, time::Duration};
    use simple_bar::ProgressBar;

    let num_iterations = 500;
    let mut bar = ProgressBar::default(num_iterations);

    for _ in 0..num_iterations {
        bar.next();
        sleep(Duration::from_millis(200));
    }
}