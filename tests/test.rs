#[test]
#[should_panic]
fn panic_out_of_bounds() {
    use simple_bar::ProgressBar;

    let num_iterations = 500u32;
    let mut bar = ProgressBar::default(num_iterations, 50, false);

    for _ in 0..num_iterations {
        bar.update();
    }

    bar.update();
}

#[test]
fn default_behaviour() {
    use simple_bar::ProgressBar;
    use std::{thread::sleep, time::Duration};
     
    let num_iterations = 500;
    let mut bar = ProgressBar::default(num_iterations, 50, false);
    
    for _ in 0..num_iterations {
         bar.update();
         sleep(Duration::from_millis(50));
    }
}

#[test]
fn eta_test() {
    use simple_bar::ProgressBar;
     
    let num_iterations = 5000000;
    let mut bar = ProgressBar::default(num_iterations, 50, true);
    
    for _ in 0..num_iterations {
         bar.update();
    }
}