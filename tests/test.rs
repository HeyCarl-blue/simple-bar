#[test]
#[should_panic]
fn panic_out_of_bounds() {
    use simple_bar::ProgressBar;

    let num_iterations = 500u32;
    let mut bar = ProgressBar::default(num_iterations, 50);

    for _ in 0..num_iterations {
        bar.next();
    }

    bar.next();
}

#[test]
fn default_behaviour() {
    use simple_bar::ProgressBar;

    let num_iterations = 500u32;
    let mut bar = ProgressBar::default(num_iterations, 50);

    for _ in 0..num_iterations {
        bar.next();
    }
}